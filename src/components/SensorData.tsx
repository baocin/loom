import type React from "react"

interface SensorDataProps {
  data: {
    temperature: number
    humidity: number
  }
}

const SensorData: React.FC<SensorDataProps> = ({ data }) => {
  return (
    <div className="bg-white p-4 rounded shadow">
      <h3 className="text-lg font-semibold mb-2">Sensor Reading</h3>
      <p>Temperature: {data.temperature}°C</p>
      <p>Humidity: {data.humidity}%</p>
    </div>
  )
}

export default SensorData

