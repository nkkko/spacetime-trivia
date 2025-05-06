"use client"

import { useEffect, useState } from "react"
import { motion, AnimatePresence } from "framer-motion"

interface EmojiBurstProps {
  emoji: string
  count?: number
  duration?: number
}

export function EmojiBurst({ emoji = "🎉", count = 12, duration = 600 }: EmojiBurstProps) {
  const [particles, setParticles] = useState<{ id: number; x: number; y: number; rotation: number }[]>([])
  const [isVisible, setIsVisible] = useState(true)

  useEffect(() => {
    const newParticles = Array.from({ length: count }).map((_, i) => {
      // Random angle in radians (30 degree spread as specified)
      const angle = (Math.random() * Math.PI) / 3 - Math.PI / 6

      // Random distance
      const distance = 50 + Math.random() * 100

      // Convert polar to cartesian coordinates
      const x = Math.cos(angle) * distance
      const y = Math.sin(angle) * distance - 100 // Start from top

      // Random rotation
      const rotation = Math.random() * 360

      return { id: i, x, y, rotation }
    })

    setParticles(newParticles)

    // Hide after duration
    const timer = setTimeout(() => {
      setIsVisible(false)
    }, duration)

    return () => clearTimeout(timer)
  }, [count, duration])

  if (!isVisible) return null

  return (
    <div className="absolute inset-0 pointer-events-none overflow-hidden">
      <AnimatePresence>
        {particles.map((particle) => (
          <motion.div
            key={particle.id}
            className="absolute left-1/2 top-1/2 text-lg"
            initial={{
              x: 0,
              y: 0,
              opacity: 1,
              scale: 0.5,
              rotate: 0,
            }}
            animate={{
              x: particle.x,
              y: particle.y,
              opacity: 0,
              scale: 1,
              rotate: particle.rotation,
            }}
            exit={{ opacity: 0 }}
            transition={{
              duration: duration / 1000,
              ease: "easeOut",
            }}
          >
            {emoji}
          </motion.div>
        ))}
      </AnimatePresence>
    </div>
  )
}
