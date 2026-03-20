import React, { useState, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Message {
  role: "user" | "assistant";
  content: string;
}

export default function App() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState("");
  const [loading, setLoading] = useState(false);
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const send = async () => {
    const text = input.trim();
    if (!text || loading) return;

    const userMsg: Message = { role: "user", content: text };
    setMessages((prev) => [...prev, userMsg]);
    setInput("");
    setLoading(true);

    try {
      const response = await invoke<string>("chat", { message: text });
      setMessages((prev) => [
        ...prev,
        { role: "assistant", content: response },
      ]);
    } catch (err) {
      setMessages((prev) => [
        ...prev,
        {
          role: "assistant",
          content: `Error: ${err instanceof Error ? err.message : String(err)}`,
        },
      ]);
    } finally {
      setLoading(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      send();
    }
  };

  return (
    <div
      style={{
        height: "100vh",
        display: "flex",
        flexDirection: "column",
        background: "#0f172a",
        color: "#e2e8f0",
        fontFamily: "'Inter', system-ui, sans-serif",
      }}
    >
      {/* Header */}
      <header
        style={{
          padding: "16px 24px",
          borderBottom: "1px solid #1e293b",
          display: "flex",
          alignItems: "center",
          gap: 12,
        }}
      >
        <span style={{ fontSize: 22, fontWeight: 700, color: "#38bdf8" }}>
          ZQ Master Bridge
        </span>
        <span
          style={{
            fontSize: 11,
            background: "#1e3a5f",
            color: "#7dd3fc",
            padding: "2px 8px",
            borderRadius: 4,
          }}
        >
          v2.0 — Control Room
        </span>
      </header>

      {/* Message list */}
      <div style={{ flex: 1, overflowY: "auto", padding: "20px 24px" }}>
        {messages.length === 0 && (
          <div
            style={{
              textAlign: "center",
              color: "#475569",
              marginTop: 80,
              fontSize: 15,
            }}
          >
            Send a message to get started.
          </div>
        )}
        {messages.map((m, i) => (
          <div
            key={i}
            style={{
              display: "flex",
              justifyContent: m.role === "user" ? "flex-end" : "flex-start",
              marginBottom: 14,
            }}
          >
            <div
              style={{
                maxWidth: "70%",
                padding: "10px 16px",
                borderRadius: 12,
                background: m.role === "user" ? "#0ea5e9" : "#1e293b",
                color: m.role === "user" ? "#fff" : "#e2e8f0",
                fontSize: 14,
                lineHeight: 1.6,
                whiteSpace: "pre-wrap",
              }}
            >
              {m.content}
            </div>
          </div>
        ))}
        {loading && (
          <div style={{ color: "#64748b", fontSize: 13, marginBottom: 8 }}>
            Assistant is typing…
          </div>
        )}
        <div ref={bottomRef} />
      </div>

      {/* Input area */}
      <div
        style={{
          padding: "12px 24px 20px",
          borderTop: "1px solid #1e293b",
          display: "flex",
          gap: 10,
          alignItems: "flex-end",
        }}
      >
        <textarea
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Message ZQ Master Bridge… (Enter to send, Shift+Enter for newline)"
          rows={1}
          style={{
            flex: 1,
            background: "#1e293b",
            color: "#e2e8f0",
            border: "1px solid #334155",
            borderRadius: 8,
            padding: "10px 14px",
            fontSize: 14,
            resize: "none",
            outline: "none",
            fontFamily: "inherit",
          }}
        />
        <button
          onClick={send}
          disabled={loading || !input.trim()}
          style={{
            background: loading || !input.trim() ? "#1e3a5f" : "#0ea5e9",
            color: "#fff",
            border: "none",
            borderRadius: 8,
            padding: "10px 20px",
            cursor: loading || !input.trim() ? "not-allowed" : "pointer",
            fontSize: 14,
            fontWeight: 600,
            transition: "background 0.2s",
          }}
        >
          Send
        </button>
      </div>
    </div>
  );
}
