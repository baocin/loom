import type React from "react"

interface NoteProps {
  data: {
    content: string
  }
}

const Note: React.FC<NoteProps> = ({ data }) => {
  return (
    <div className="bg-yellow-100 p-4 rounded shadow">
      <h3 className="text-lg font-semibold mb-2">Note</h3>
      <p>{data.content}</p>
    </div>
  )
}

export default Note

