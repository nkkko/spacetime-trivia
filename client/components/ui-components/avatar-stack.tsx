"use client"

import { motion } from "framer-motion"
import { User } from "lucide-react"

interface AvatarStackProps {
  avatars: { id: string; name: string; image?: string }[]
  maxVisible?: number
  showAnimation?: boolean
  accentColor?: string
}

export function AvatarStack({
  avatars,
  maxVisible = 5,
  showAnimation = false,
  accentColor = "#FF2D9B",
}: AvatarStackProps) {
  const visibleAvatars = avatars.slice(0, maxVisible)
  const remainingCount = Math.max(0, avatars.length - maxVisible)

  return (
    <div className="flex -space-x-2">
      {visibleAvatars.map((avatar, index) => (
        <motion.div
          key={avatar.id}
          className={`relative w-10 h-10 rounded-full bg-slate-800 border-2 border-slate-700 flex items-center justify-center overflow-hidden ${showAnimation && index === 0 ? "combo-glow" : ""}`}
          initial={showAnimation ? { scale: 0, rotate: -10 } : {}}
          animate={showAnimation ? { scale: 1, rotate: 0 } : {}}
          transition={{
            type: "spring",
            stiffness: 300,
            damping: 15,
            delay: index * 0.1,
          }}
          style={showAnimation && index === 0 ? { borderColor: accentColor } : {}}
        >
          {avatar.image ? (
            <img src={avatar.image || "/placeholder.svg"} alt={avatar.name} className="w-full h-full object-cover" />
          ) : (
            <User size={16} className="text-slate-400" />
          )}
        </motion.div>
      ))}

      {remainingCount > 0 && (
        <motion.div
          className="w-10 h-10 rounded-full bg-slate-800 border-2 border-slate-700 flex items-center justify-center text-xs font-medium"
          initial={showAnimation ? { scale: 0 } : {}}
          animate={showAnimation ? { scale: 1 } : {}}
          transition={{ delay: visibleAvatars.length * 0.1 }}
        >
          +{remainingCount}
        </motion.div>
      )}
    </div>
  )
}
