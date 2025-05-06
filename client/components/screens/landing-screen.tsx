"use client"

import { motion } from "framer-motion"
import { TopicCard } from "@/components/ui-components/topic-card"
import { Users } from "lucide-react"
import { useMobile } from "@/hooks/use-mobile"

interface Topic {
  id: number
  title: string
  color: string
  playerCount: number
}

interface LandingScreenProps {
  topics: Topic[]
  onCreateLobby: () => void
}

export function LandingScreen({ topics, onCreateLobby }: LandingScreenProps) {
  const isMobile = useMobile()

  // Calculate total player count
  const totalPlayers = topics.reduce((sum, topic) => sum + topic.playerCount, 0)

  return (
    <div className="min-h-screen p-4 md:p-8">
      <motion.div
        className="max-w-6xl mx-auto"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 0.5 }}
      >
        <div className="text-center mb-12">
          <motion.h1
            className="text-4xl md:text-6xl font-heading font-bold mb-4 bg-gradient-to-r from-pink-500 to-blue-500 text-transparent bg-clip-text"
            initial={{ y: -50, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ duration: 0.5, delay: 0.2 }}
          >
            Project North Star
          </motion.h1>

          <motion.p
            className="text-lg md:text-xl text-slate-300 max-w-2xl mx-auto"
            initial={{ y: 20, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ duration: 0.5, delay: 0.3 }}
          >
            A massively-multiplayer trivia party that feels as live & reactive as Twitch chat
          </motion.p>

          <motion.div
            className="mt-4 flex items-center justify-center gap-2 text-slate-400"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 0.5, delay: 0.4 }}
          >
            <Users size={16} />
            <span>{totalPlayers.toLocaleString()} players online</span>
          </motion.div>
        </div>

        <motion.div
          className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 md:gap-6"
          initial={{ y: 20, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          transition={{ duration: 0.5, delay: 0.5 }}
        >
          {topics.map((topic) => (
            <TopicCard
              key={topic.id}
              title={topic.title}
              color={topic.color}
              playerCount={topic.playerCount}
              onClick={onCreateLobby}
            />
          ))}
        </motion.div>

        <motion.div
          className="mt-8 text-center"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 0.5, delay: 0.7 }}
        >
          <button
            className="px-6 py-3 rounded-full bg-gradient-to-r from-pink-500 to-purple-600 text-white font-bold text-lg shadow-lg hover:shadow-xl transition-all duration-200 hover:scale-105"
            onClick={onCreateLobby}
          >
            Create New Lobby
          </button>
        </motion.div>
      </motion.div>
    </div>
  )
}
