import "./Square.css";
import { useState, useEffect, useRef } from "react";

type SquareProps = {
    width: number,
    height: number,
    rot: number,
    focusIdx: number | null,
    setFocusIdx: (action: number) => void,
    onClickSquare: () => void,
}

function rotation(x: number, y: number, rad: number) {
    const sx = Math.cos(rad);
    const sy = Math.sin(rad);
    return [x * sx - sy * y, y * sx + x * sy];
}

function Square(props: SquareProps) {
    const w = props.width;
    const h = props.height;
    const size = 0.7 * w;
    const c = w / 2;
    // const [idx, setIdx] = useState(0);

    const onMove = (e: React.MouseEvent) => {
        const svgRect = e.currentTarget.getBoundingClientRect();
        const pos = rotation(
            e.pageX - svgRect["x"] - c,
            e.pageY - svgRect["y"] - c,
            -props.rot
        )
        // const x = e.pageX - svgRect["x"];
        // const y = e.pageY - svgRect["y"];
        const x = pos[0] + c;
        const y = pos[1] + c;
        const xIdx = Math.floor(4 * (x + 7.5) / size) - 1;
        const yIdx = Math.floor(4 * (y + 7.5) / size) - 1;
        if (xIdx < 0 || xIdx > 3) { return }
        if (yIdx < 0 || yIdx > 3) { return }
        const action = yIdx * 4 + xIdx;
        if (action !== props.focusIdx) {
            props.setFocusIdx(action);
        }
    }

    return <svg
        width={w}
        height={h}
        onMouseMove={onMove}
        onMouseDown={props.onClickSquare}
        style={{ background: "#000055" }}
    >
        {(() => {
            let lines = [];
            for (let i = 0; i < 5; i++) {
                const pos1 = rotation(
                    size * (-2 + i) / 4,
                    size * (-1) / 2,
                    props.rot
                );
                const pos2 = rotation(
                    size * (-2 + i) / 4,
                    size / 2,
                    props.rot
                )
                lines.push(<line className="squareLine" x1={pos1[0] + c} y1={pos1[1] + c} x2={pos2[0] + c} y2={pos2[1] + c} />)
                lines.push(<line className="squareLine" x1={-pos1[1] + c} y1={pos1[0] + c} x2={-pos2[1] + c} y2={pos2[0] + c} />)
            }
            if (props.focusIdx != null) {
                const pos = rotation(
                    0.7 * w * (1 + -4 + 2 * (props.focusIdx % 4)) / 8,
                    0.7 * h * (1 + -4 + 2 * Math.floor(props.focusIdx / 4)) / 8,
                    props.rot
                )
                lines.push(<circle cx={pos[0] + c} cy={pos[1] + c} r={10} fill="white" />)
            }
            return <g>{lines}</g>
        })()}
    </svg>;
}

export default Square;