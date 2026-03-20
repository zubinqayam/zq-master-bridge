"""
ZQ Master Bridge — Agent Router (Control Room V2 sidecar)

Lightweight async router that dispatches tasks to registered agents.
Run standalone:  python -m agents.core.router
"""

from __future__ import annotations

import asyncio
import logging
import os
from dataclasses import dataclass, field
from typing import Any, Callable, Coroutine

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
)
logger = logging.getLogger("zq.router")

# ---------------------------------------------------------------------------
# Agent registry
# ---------------------------------------------------------------------------

AgentHandler = Callable[[str, dict[str, Any]], Coroutine[Any, Any, str]]

_REGISTRY: dict[str, AgentHandler] = {}


def register(name: str) -> Callable[[AgentHandler], AgentHandler]:
    """Decorator to register an agent handler by name."""

    def decorator(fn: AgentHandler) -> AgentHandler:
        _REGISTRY[name] = fn
        logger.info("Registered agent: %s", name)
        return fn

    return decorator


# ---------------------------------------------------------------------------
# Built-in agents (placeholders — replace with real implementations)
# ---------------------------------------------------------------------------


@register("echo")
async def echo_agent(task: str, _ctx: dict[str, Any]) -> str:
    """Echo agent — returns the task string unchanged."""
    return task


@register("summarize")
async def summarize_agent(task: str, _ctx: dict[str, Any]) -> str:
    """Placeholder summarizer — replace with real LLM call."""
    await asyncio.sleep(0.1)
    return f"[Summary of: {task[:80]}{'…' if len(task) > 80 else ''}]"


# ---------------------------------------------------------------------------
# Router core
# ---------------------------------------------------------------------------


def get_heartbeat_interval() -> float:
    """Read ZQ_HEARTBEAT_INTERVAL from the environment with validation.

    Falls back to 5.0 seconds if the variable is unset, invalid, or non-positive.
    """
    raw = os.getenv("ZQ_HEARTBEAT_INTERVAL")
    default = 5.0
    if not raw:
        return default
    try:
        value = float(raw)
        if value <= 0:
            logger.warning(
                "ZQ_HEARTBEAT_INTERVAL must be positive; got %r. Falling back to %s.",
                raw,
                default,
            )
            return default
        return value
    except (TypeError, ValueError):
        logger.warning(
            "Invalid ZQ_HEARTBEAT_INTERVAL %r; expected a float. Falling back to %s.",
            raw,
            default,
        )
        return default


@dataclass
class RouterConfig:
    heartbeat_interval: float = field(default_factory=get_heartbeat_interval)
    max_concurrency: int = int(os.getenv("ZQ_MAX_CONCURRENCY", "16"))


@dataclass
class Router:
    config: RouterConfig = field(default_factory=RouterConfig)
    _semaphore: asyncio.Semaphore = field(init=False)

    def __post_init__(self) -> None:
        self._semaphore = asyncio.Semaphore(self.config.max_concurrency)

    async def dispatch(
        self, agent_name: str, task: str, ctx: dict[str, Any] | None = None
    ) -> str:
        """Route *task* to the named agent and return its response."""
        handler = _REGISTRY.get(agent_name)
        if handler is None:
            raise KeyError(f"Unknown agent: '{agent_name}'")

        async with self._semaphore:
            logger.debug("Dispatching to %s: %s", agent_name, task[:60])
            result = await handler(task, ctx or {})
            logger.debug("Result from %s: %s", agent_name, result[:60])
            return result

    async def _heartbeat(self) -> None:
        while True:
            logger.info(
                "Router heartbeat — registered agents: %s",
                list(_REGISTRY.keys()),
            )
            await asyncio.sleep(self.config.heartbeat_interval)

    async def run(self) -> None:
        """Start the router event loop (runs indefinitely)."""
        logger.info("ZQ Router starting…")
        await self._heartbeat()


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------


async def main() -> None:
    router = Router()
    await router.run()


if __name__ == "__main__":
    asyncio.run(main())
