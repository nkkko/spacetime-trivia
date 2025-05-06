"use client"

import { useState } from "react"
import { motion } from "framer-motion"
import { ChatBubble } from "@/components/ui-components/chat-bubble"
import { Send, ArrowLeft, Check, Sparkles } from "lucide-react"
import { useMobile } from "@/hooks/use-mobile"

interface AgentStudioScreenProps {
  onBack: () => void
}

export function AgentStudioScreen({ onBack }: AgentStudioScreenProps) {
  const isMobile = useMobile()
  const [message, setMessage] = useState("")
  const [agentMessages, setAgentMessages] = useState<{ id: number; text: string; isAgent: boolean }[]>([
    {
      id: 1,
      text: "Welcome to Agent Studio! I can help you create custom trivia questions. What topic would you like to explore?",
      isAgent: true,
    },
  ])
  const [isAgentTyping, setIsAgentTyping] = useState(false)
  const [generatedQuestions, setGeneratedQuestions] = useState<
    { id: number; text: string; answers: string[]; correctIndex: number }[]
  >([])
  const [showQuestions, setShowQuestions] = useState(false)

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
          text: "I've generated some trivia questions based on your request. Would you like to see them?",
          isAgent: true,
        },
      ])

      // Generate mock questions
      const mockQuestions = [
        {
          id: 1,
          text: "Which planet in our solar system has the most moons?",
          answers: ["Jupiter", "Saturn", "Uranus", "Neptune"],
          correctIndex: 1,
        },
        {
          id: 2,
          text: "What is the largest species of shark?",
          answers: ["Great White Shark", "Whale Shark", "Hammerhead Shark", "Tiger Shark"],
          correctIndex: 1,
        },
        {
          id: 3,
          text: "Which element has the chemical symbol 'Au'?",
          answers: ["Silver", "Gold", "Aluminum", "Argon"],
          correctIndex: 1,
        },
      ]

      setGeneratedQuestions(mockQuestions)
      setShowQuestions(true)
    }, 2000)
  }

  return (
    <div className="min-h-screen p-4 md:p-8">
      <div className="max-w-6xl mx-auto">
        <motion.div
          className="glass-card p-6 mb-6"
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.3 }}
        >
          <div className="flex items-center gap-4">
            <button className="p-2 rounded-full hover:bg-slate-800/50" onClick={onBack}>
              <ArrowLeft size={20} />
            </button>
            <h1 className="text-2xl md:text-3xl font-heading font-bold">Agent Studio</h1>
          </div>
        </motion.div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <motion.div
            className="glass-card p-6 lg:col-span-2 h-[600px] flex flex-col"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.3, delay: 0.1 }}
          >
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
          </motion.div>

          <motion.div
            className="glass-card p-6 h-[600px] overflow-y-auto"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.3, delay: 0.2 }}
          >
            <h2 className="text-xl font-heading font-bold mb-4 flex items-center gap-2">
              <Sparkles size={18} className="text-pink-500" />
              Generated Questions
            </h2>

            {!showQuestions ? (
              <div className="flex flex-col items-center justify-center h-[calc(100%-40px)] text-center text-slate-400">
                <Sparkles size={40} className="mb-4 text-slate-600" />
                <p>Chat with the agent to generate custom trivia questions</p>
              </div>
            ) : (
              <div className="space-y-6">
                {generatedQuestions.map((question) => (
                  <div key={question.id} className="glass-card p-4">
                    <div className="font-bold mb-2">{question.text}</div>
                    <div className="space-y-2 mb-2">
                      {question.answers.map((answer, index) => (
                        <div
                          key={index}
                          className={`p-2 rounded-lg text-sm flex justify-between items-center ${index === question.correctIndex ? "bg-green-500/20 border border-green-500/30" : "bg-slate-800/50"}`}
                        >
                          <span>{answer}</span>
                          {index === question.correctIndex && <Check size={16} className="text-green-500" />}
                        </div>
                      ))}
                    </div>
                  </div>
                ))}

                <button className="w-full py-2 rounded-lg bg-pink-500 text-white font-medium">
                  Publish Question Set
                </button>
              </div>
            )}
          </motion.div>
        </div>
      </div>
    </div>
  )
}
