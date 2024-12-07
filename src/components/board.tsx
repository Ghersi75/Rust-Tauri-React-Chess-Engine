import { useState } from "react"

export default function Board() {
  const [squareSize, _] = useState(100);
  return(
    <div className="bg-zinc-800 flex flex-col rounded" style={{
      height: squareSize * 8,
      width: squareSize * 8
    }}>
      {
        // Row
        [...Array(8).keys()].map((row, rowIdx) => {
          return(
            <div className="flex flex-row" key={rowIdx}>
              {[...Array(8).keys()].map((col, colIdx) => {
                // Alternate light and dark color on grid
                // This checks if current row is even and assigns 0 offset if so, 1 otherwise
                let evenRow = (row % 2 == 0 ? 0 : 1)
                // Makes all even numbers light, but has 1 offset for even rows
                // Easiest way I came up with honestly
                let light = (row * 8 + col + evenRow) % 2 == 0;
                return(
                  // Col
                  <div key={colIdx} className={`flex justify-center items-center ${light ? "bg-zinc-700" : ""}`} style={{
                    height: squareSize,
                    width: squareSize
                  }}>
                    {row * 8 + col}
                  </div>
                )
              })}
            </div>
          )
        })
      }
    </div>
  )
}