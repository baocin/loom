import type React from "react"

interface LinkProps {
  data: {
    url: string
    title: string
  }
}

const Link: React.FC<LinkProps> = ({ data }) => {
  return (
    <div className="bg-white p-4 rounded shadow">
      <h3 className="text-lg font-semibold mb-2">Link</h3>
      <a href={data.url} target="_blank" rel="noopener noreferrer" className="text-blue-600 hover:underline">
        {data.title}
      </a>
    </div>
  )
}

export default Link

