import { useState, useEffect, useRef } from "react";
import { Canvas, useFrame } from '@react-three/fiber';
import { DirectionalLight, Mesh, Vector3, Color } from 'three';
import { Plane } from '@react-three/drei';
import { invoke } from "@tauri-apps/api/tauri";
import "./Board.css";
// import View3D from "./View3D";
import View3d from "./View3D";
import Square from "./Square";

const testBoard = "XXOOOOXXOOXXXOXOXXXO-XXO-X--O---X-XO-O---X--X---O------------T--"
const coef = 0.01;

function Board() {
    const [height, setHeight] = useState(window.innerHeight);
    const [width, setWidth] = useState(window.innerWidth);
    const [message, setMessage] = useState("default");
    const [rot, setRot] = useState(0);
    const [mouseView, setMouseView] = useState({ x: 0, clicked: false });
    const [board, setBoard] = useState(testBoard);

    const boardSize = (height < width) ? height * 0.5 : width * 0.5;

    const [focus, setFocus] = useState<number | null>(null);

    const onChangeFocus = (action: number) => {
        if (board[action] == "-") {
            setFocus(action);
        } else if (board[action + 16] == "-") {
            setFocus(action + 16);
        } else if (board[action + 32] == "-") {
            setFocus(action + 32);
        } else if (board[action + 48] == "-") {
            setFocus(action + 48);
        } else {
            setFocus(null);
        }
    }

    const onPointerDown = (event: React.PointerEvent<HTMLDivElement>) => {
        setMouseView({
            x: event.pageX,
            clicked: true
        });
        console.log(mouseView);
    }
    const onPointerUp = (event: React.PointerEvent<HTMLDivElement>) => {
        setMouseView({
            x: mouseView.x,
            clicked: false
        });
        console.log("pointer up")
    }
    const onPointerMove = (event: React.PointerEvent<HTMLDivElement>) => {
        if (!mouseView.clicked) {
            return;
        }
        setRot(rot - coef * (event.pageX - mouseView.x));
        setMouseView({
            x: event.pageX,
            clicked: true
        });
        console.log(rot, mouseView.x);
    }

    const onResize = () => {
        setHeight(window.innerHeight);
        setWidth(window.innerWidth);
        setMessage("resized");
    }
    useEffect(() => {
        window.addEventListener('resize', onResize);
    })

    return <div>
        {/* <Canv3d width={boardSize} height={boardSize} /> */}
        <div className="boardArea">

            <View3d width={boardSize} height={boardSize} board={board} rot={rot} focusIdx={focus}
                onPointerDown={onPointerDown} onPointerMove={onPointerMove} onPointerUp={onPointerUp} />
            <Square width={height - boardSize} height={height - boardSize} rot={rot} setFocusIdx={onChangeFocus} />
        </div>
        {message}
    </div>
}

export default Board