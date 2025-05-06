"use client"

import { motion } from "framer-motion"

interface ChatBubbleProps {
  message: string
  username: string
  isAgent?: boolean
  accentColor?: string
}

export function ChatBubble({ message, username, isAgent = false, accentColor = "#FF2D9B" }: ChatBubbleProps) {
  return (
    <motion.div
      className="mb-2"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.2 }}
    >
      <div className="flex items-start gap-2">
        <div
          className={`px-3 py-2 rounded-2xl max-w-xs ${isAgent ? "bg-slate-800" : "glass-card"}`}
          style={isAgent ? { backgroundColor: `${accentColor}30` } : {}}
        >
          <div className="text-xs font-bold mb-1" style={isAgent ? { color: accentColor } : {}}>
            {username}
          </div>
          <div className="text-sm">{message}</div>
        </div>
      </div>
    </motion.div>
  )
}
