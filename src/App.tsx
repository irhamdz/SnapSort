import { useState } from 'react'

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="min-h-screen bg-gray-950 text-white p-8">
      <h1 className="text-4xl font-bold mb-4">SnapSort</h1>
      <p className="text-gray-400 mb-8">
        Screenshot organization made automatic
      </p>
      <div className="bg-gray-900 rounded-lg p-6">
        <p className="text-lg">Count: {count}</p>
        <button
          className="mt-4 px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded"
          onClick={() => setCount((c) => c + 1)}
        >
          Increment
        </button>
      </div>
    </div>
  )
}

export default App