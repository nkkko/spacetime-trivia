"use client"

import { useState } from "react"
import { motion, AnimatePresence } from "framer-motion"
import { X, Sparkles, Send } from "lucide-react"
import { ChatBubble } from "@/components/ui-components/chat-bubble"

interface CreateLobbyModalProps {
  onClose: () => void
  onCreateLobby: () => void
}

export function CreateLobbyModal({ onClose, onCreateLobby }: CreateLobbyModalProps) {
  const [activeTab, setActiveTab] = useState<"topics" | "agent">("topics")
  const [selectedTopics, setSelectedTopics] = useState<string[]>([])
  const [message, setMessage] = useState("")
  const [agentMessages, setAgentMessages] = useState<{ id: number; text: string; isAgent: boolean }[]>([])
  const [isAgentTyping, setIsAgentTyping] = useState(false)

  const topics = [
    "90s Pop",
    "Science",
    "Movies",
    "History",
    "Sports",
    "Tech",
    "Geography",
    "Food",
    "Art",
    "Literature",
    "Music",
    "TV Shows",
  ]

  const handleTopicToggle = (topic: string) => {
    if (selectedTopics.includes(topic)) {
      setSelectedTopics(selectedTopics.filter((t) => t !== topic))
    } else {
      setSelectedTopics([...selectedTopics, topic])
    }
  }

  const handleSendMessage = () => {
    if (!message.trim()) return

    // Add user message
    setAgentMessages([...agentMessages, { id: Date.now(), text: message, isAgent: false }])
    setMessage("")

    // Simulate agent typing
    setIsAgentTyping(true)

    // Simulate agent response after a delay
    setTimeout(() => {
      setIsAgentTyping(false)
      setAgentMessages((prev) => [
        ...prev,
        {
          id: Date.now(),
          text: "I've created a custom question set based on your interests! Ready to start the game?",
          isAgent: true,
        },
      ])
    }, 2000)
  }

  return (
    <AnimatePresence>
      <div className="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center p-4">
        <motion.div
          className="glass-card w-full max-w-2xl shadow-glass overflow-hidden"
          initial={{ opacity: 0, scale: 0.9 }}
          animate={{ opacity: 1, scale: 1 }}
          exit={{ opacity: 0, scale: 0.9 }}
          transition={{ duration: 0.2 }}
        >
          <div className="flex justify-between items-center p-6 border-b border-white/10">
            <h2 className="text-2xl font-heading font-bold">Create Lobby</h2>
            <button className="p-2 rounded-full hover:bg-slate-800/50" onClick={onClose}>
              <X size={20} />
            </button>
          </div>

          <div className="p-6">
            <div className="flex border-b border-white/10 mb-6">
              <button
                className={`px-4 py-2 font-medium ${activeTab === "topics" ? "border-b-2 border-pink-500 text-white" : "text-slate-400"}`}
                onClick={() => setActiveTab("topics")}
              >
                Select Topics
              </button>
              <button
                className={`px-4 py-2 font-medium flex items-center gap-2 ${activeTab === "agent" ? "border-b-2 border-pink-500 text-white" : "text-slate-400"}`}
                onClick={() => setActiveTab("agent")}
              >
                <Sparkles size={16} />
                Ask Agent
              </button>
            </div>

            {activeTab === "topics" ? (
              <div>
                <p className="text-slate-300 mb-4">Select topics for your trivia game:</p>

                <div className="flex flex-wrap gap-2 mb-6">
                  {topics.map((topic) => (
                    <button
                      key={topic}
                      className={`px-3 py-1 rounded-full text-sm font-medium transition-all duration-200 ${
                        selectedTopics.includes(topic)
                          ? "bg-pink-500 text-white"
                          : "bg-slate-800 text-slate-300 hover:bg-slate-700"
                      }`}
                      onClick={() => handleTopicToggle(topic)}
                    >
                      {topic}
                    </button>
                  ))}
                </div>
              </div>
            ) : (
              <div className="h-80 flex flex-col">
                <div className="flex-1 overflow-y-auto mb-4 p-2">
                  {agentMessages.map((msg) => (
                    <ChatBubble
                      key={msg.id}
                      message={msg.text}
                      username={msg.isAgent ? "Trivia Agent" : "You"}
                      isAgent={msg.isAgent}
                    />
                  ))}

                  {isAgentTyping && (
                    <div className="flex gap-1 px-3 py-2 rounded-2xl bg-slate-800/50 w-16">
                      <div
                        className="w-2 h-2 rounded-full bg-pink-500 animate-bounce"
                        style={{ animationDelay: "0ms" }}
                      ></div>
                      <div
                        className="w-2 h-2 rounded-full bg-pink-500 animate-bounce"
                        style={{ animationDelay: "150ms" }}
                      ></div>
                      <div
                        className="w-2 h-2 rounded-full bg-pink-500 animate-bounce"
                        style={{ animationDelay: "300ms" }}
                      ></div>
                    </div>
                  )}
                </div>

                <div className="flex gap-2">
                  <input
                    type="text"
                    className="flex-1 bg-slate-800/50 border border-white/10 rounded-lg px-4 py-2 text-white focus:outline-none focus:ring-2 focus:ring-pink-500"
                    placeholder="Ask the agent to create questions..."
                    value={message}
                    onChange={(e) => setMessage(e.target.value)}
                    onKeyDown={(e) => e.key === "Enter" && handleSendMessage()}
                  />
                  <button className="p-2 rounded-lg bg-pink-500 text-white" onClick={handleSendMessage}>
                    <Send size={20} />
                  </button>
                </div>
              </div>
            )}
          </div>

          <div className="p-6 border-t border-white/10 flex justify-end">
            <button className="px-6 py-2 rounded-lg bg-slate-800 text-white mr-2 hover:bg-slate-700" onClick={onClose}>
              Cancel
            </button>
            <button
              className="px-6 py-2 rounded-lg bg-gradient-to-r from-pink-500 to-purple-600 text-white font-medium"
              onClick={onCreateLobby}
              disabled={activeTab === "topics" && selectedTopics.length === 0}
            >
              Create Lobby
            </button>
          </div>
        </motion.div>
      </div>
    </AnimatePresence>
  )
}
