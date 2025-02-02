import type React from "react"
import type { TimelineItem } from "../types"
import TimelineEvent from "./TimelineEvent"

interface TimelineProps {
  items: TimelineItem[]
}

const Timeline: React.FC<TimelineProps> = ({ items }) => {
  return (
    <div className="relative max-w-4xl mx-auto">
      <div className="absolute left-0 top-0 bottom-0 w-0.5 bg-black"></div>
      {items.map((item, index) => (
        <TimelineEvent key={item.id} item={item} isLast={index === items.length - 1} />
      ))}
    </div>
  )
}

export default Timeline

