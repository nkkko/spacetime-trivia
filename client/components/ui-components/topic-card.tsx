"use client"

import { useState, useEffect } from "react"
import { motion } from "framer-motion"
import { Users } from "lucide-react"

interface TopicCardProps {
  title: string
  color: string
  playerCount: number
  onClick: () => void
}

export function TopicCard({ title, color, playerCount, onClick }: TopicCardProps) {
  const [count, setCount] = useState(playerCount)
  const [pulse, setPulse] = useState(false)

  // Simulate live player count updates
  useEffect(() => {
    const interval = setInterval(() => {
      const change = Math.floor(Math.random() * 10) - 3 // Random change between -3 and +6
      const newCount = Math.max(0, count + change)
      setCount(newCount)
      setPulse(true)

      setTimeout(() => setPulse(false), 500)
    }, 2000)

    return () => clearInterval(interval)
  }, [count])

  return (
    <motion.div
      className="glass-card overflow-hidden cursor-pointer"
      style={{
        background: `linear-gradient(135deg, ${color}20, transparent)`,
        borderColor: `${color}40`,
      }}
      whileHover={{
        scale: 1.05,
        rotate: 1,
        boxShadow: `0 10px 40px ${color}30`,
      }}
      whileTap={{ scale: 0.98 }}
      onClick={onClick}
    >
      <div className="p-6">
        <h3 className="text-2xl font-heading font-bold mb-4" style={{ color }}>
          {title}
        </h3>

        <div className="flex items-center mt-4">
          <motion.div
            className={`flex items-center gap-1 text-sm px-2 py-1 rounded-full ${pulse ? "animate-pulse" : ""}`}
            style={{ backgroundColor: `${color}30` }}
          >
            <Users size={14} />
            <motion.span
              key={count}
              initial={{ opacity: 0, y: -10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.2 }}
            >
              {count.toLocaleString()}
            </motion.span>
          </motion.div>
        </div>
      </div>
    </motion.div>
  )
}
