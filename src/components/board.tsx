import { useEffect, useState } from "react"
import { parseFen } from "../helpers/fenParser"

type pieceSvgType = string | null;

export default function Board() {
  const [squareSize, _] = useState(100);
  const [pieceSvg, setPieceSvg] = useState<pieceSvgType[]>([]);
  const [selected, setSelected] = useState(-1);

  useEffect(() => {
    setPieceSvg(parseFen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"))
  }, [])

  function onClickGrid(e: React.MouseEvent<HTMLImageElement, MouseEvent>, idx: number) {
    e.stopPropagation();
    // deselect
    if (selected == idx) {
      setSelected(-1)
    } else {
      setSelected(idx);
    }
  }

  function onClickNonPiece() {
    setSelected(-1);
  }

  return(
    <div className="rounded flex flex-col rounded overflow-hidden m-12" style={{
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
                let currSvgIdx = row * 8 + col;
                let color;

                if ( light ) {
                  if ( currSvgIdx == selected ) {
                    color = "bg-zinc-500";
                  } else {
                    color = "bg-zinc-700";
                  }
                } else {
                  if ( currSvgIdx == selected ) {
                    color = "bg-zinc-600";
                  } else {
                    color = "bg-zinc-800";
                  }
                }

                return(
                  // Col
                  <div key={colIdx} className={`flex justify-center items-center ${color}`} style={{
                    height: squareSize,
                    width: squareSize
                  }} onClick={onClickNonPiece}>
                    {
                      pieceSvg[currSvgIdx] &&
                      <img src={pieceSvg[currSvgIdx]} className="cursor-pointer" style={{
                        height: squareSize - 10
                      }} onClick={(e) => onClickGrid(e, currSvgIdx)}/>
                    }
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