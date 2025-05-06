"use client"

import { useState, useEffect } from "react"
import { motion } from "framer-motion"
import { ChatBubble } from "@/components/ui-components/chat-bubble"
import { Send, Users } from "lucide-react"
import { useMobile } from "@/hooks/use-mobile"

interface WaitingRoomScreenProps {
  onStartGame: () => void
}

export function WaitingRoomScreen({ onStartGame }: WaitingRoomScreenProps) {
  const isMobile = useMobile()
  const [players, setPlayers] = useState<{ id: string; name: string; image?: string }[]>([])
  const [messages, setMessages] = useState<{ id: number; text: string; username: string }[]>([])
  const [newMessage, setNewMessage] = useState("")

  // Simulate players joining
  useEffect(() => {
    const names = [
      "Alex",
      "Taylor",
      "Jordan",
      "Casey",
      "Riley",
      "Morgan",
      "Jamie",
      "Quinn",
      "Avery",
      "Skyler",
      "Dakota",
      "Reese",
      "Finley",
      "Rowan",
    ]

    const interval = setInterval(() => {
      if (players.length < 20) {
        const randomName = names[Math.floor(Math.random() * names.length)]
        const newPlayer = {
          id: `player-${Date.now()}`,
          name: `${randomName}${Math.floor(Math.random() * 1000)}`,
        }

        setPlayers((prev) => [...prev, newPlayer])

        // Add join message
        setMessages((prev) => [...prev, { id: Date.now(), text: "joined the lobby", username: newPlayer.name }])
      } else {
        clearInterval(interval)
      }
    }, 2000)

    return () => clearInterval(interval)
  }, [players])

  const handleSendMessage = () => {
    if (!newMessage.trim()) return

    setMessages((prev) => [...prev, { id: Date.now(), text: newMessage, username: "You" }])

    setNewMessage("")
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
          <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
            <div>
              <h1 className="text-2xl md:text-3xl font-heading font-bold">Waiting Room</h1>
              <p className="text-slate-400">90s Pop Trivia</p>
            </div>

            <div className="flex items-center gap-4">
              <div className="flex items-center gap-2">
                <Users size={16} className="text-pink-500" />
                <span>{players.length} players</span>
              </div>

              <button
                className="px-6 py-2 rounded-lg bg-gradient-to-r from-pink-500 to-purple-600 text-white font-medium"
                onClick={onStartGame}
              >
                Start Game
              </button>
            </div>
          </div>
        </motion.div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <motion.div
            className="glass-card p-6 md:col-span-2 h-[500px] flex flex-col"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.3, delay: 0.1 }}
          >
            <h2 className="text-xl font-heading font-bold mb-4">Chat</h2>

            <div className="flex-1 overflow-y-auto mb-4 space-y-2">
              {messages.map((message) => (
                <ChatBubble key={message.id} message={message.text} username={message.username} />
              ))}
            </div>

            <div className="flex gap-2">
              <input
                type="text"
                className="flex-1 bg-slate-800/50 border border-white/10 rounded-lg px-4 py-2 text-white focus:outline-none focus:ring-2 focus:ring-pink-500"
                placeholder="Type a message..."
                value={newMessage}
                onChange={(e) => setNewMessage(e.target.value)}
                onKeyDown={(e) => e.key === "Enter" && handleSendMessage()}
              />
              <button className="p-2 rounded-lg bg-pink-500 text-white" onClick={handleSendMessage}>
                <Send size={20} />
              </button>
            </div>
          </motion.div>

          <motion.div
            className="glass-card p-6 h-[500px] overflow-y-auto"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.3, delay: 0.2 }}
          >
            <h2 className="text-xl font-heading font-bold mb-4">Players</h2>

            <div className="space-y-3">
              {players.map((player) => (
                <motion.div
                  key={player.id}
                  className="flex items-center gap-3 p-2 rounded-lg hover:bg-slate-800/50"
                  initial={{ opacity: 0, x: -20 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ duration: 0.2 }}
                >
                  <div className="w-8 h-8 rounded-full bg-slate-800 flex items-center justify-center">
                    <Users size={14} className="text-slate-400" />
                  </div>
                  <span>{player.name}</span>
                </motion.div>
              ))}
            </div>
          </motion.div>
        </div>
      </div>
    </div>
  )
}
