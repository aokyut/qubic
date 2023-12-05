import { useState, useEffect, useRef } from "react";
import { Canvas, useFrame } from '@react-three/fiber';
import { DirectionalLight, Mesh, Vector3, Color } from 'three';
import { Plane } from '@react-three/drei';
import { invoke } from "@tauri-apps/api/tauri";
import "./Board.css";
// import View3D from "./View3D";
import View3d from "./View3D";
import Square from "./Square";
import SearchRow from "./SearchRow";
import MctsRow from "./MctsRow";

const testBoard = "----------------------------------------------------------------B";
const coef = 0.01;
const mctsSearchN = 1000;

type MateRow = {
    depth: number,
    action: number,
}

type MctsScoreArray = Array<MctsScore>;
type MctsScore = {
    action: number,
    score: number,
    q: number,
    na: number,
    n: number
}

function Board() {
    const [height, setHeight] = useState(window.innerHeight);
    const [width, setWidth] = useState(window.innerWidth);
    const [message, setMessage] = useState("default");
    const [rot, setRot] = useState(0);
    const [mouseView, setMouseView] = useState({ x: 0, clicked: false });
    const [board, setBoard] = useState(testBoard);

    const boardSize = (height < width) ? height * 0.5 : width * 0.5;

    const [focus, setFocus] = useState<number | null>(null);
    const [mateRow, setMateRow] = useState<MateRow>({ depth: 0, action: -1 });
    const [mctsRows, setMctsRows] = useState<MctsScoreArray>([]);
    const [intervalMcts, setIntervalMcts] = useState<number | null>(null);

    const onChangeFocus = (action: number | null) => {
        if (action == null) {
            return;
        }
        if (board[action] == "-") {
            setFocus(action);
            console.log(action + "is set");
        } else if (board[action + 16] == "-") {
            setFocus(action + 16);
            console.log(action + 16 + "is set");
        } else if (board[action + 32] == "-") {
            setFocus(action + 32);
            console.log(action + 32 + "is set");
        } else if (board[action + 48] == "-") {
            setFocus(action + 48);
            console.log(action + 48 + "is set");
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

    async function onClickAction(action: number | null) {
        if (action == null) {
            return;
        }
        const resBoard: string = await invoke("board_action", { action });
        setBoard(resBoard);

        if (resBoard[action] == "-") {
            setFocus(action);
            console.log(action + "is set");
        } else if (resBoard[action + 16] == "-") {
            setFocus(action + 16);
            console.log(action + 16 + "is set");
        } else if (resBoard[action + 32] == "-") {
            setFocus(action + 32);
            console.log(action + 32 + "is set");
        } else if (resBoard[action + 48] == "-") {
            setFocus(action + 48);
            console.log(action + 48 + "is set");
        } else {
            setFocus(null);
        }
    }

    async function onClickNext() {
        setBoard(await invoke("board_next"));
    }

    async function onClickBack() {
        setBoard(await invoke("board_back"));
    }

    async function onClickInit() {
        setBoard(await invoke("board_init"));
    }

    async function onClickLast() {
        setBoard(await invoke("board_last"));
    }

    const onClickMate = () => {
        invoke("search_mate").then(res => {
            console.log(res);
            let castRes = res as MateRow;
            setMateRow(castRes);
        });
    }

    function onClickRunMcts() {
        console.log("hoge");
        const id = setInterval(
            () => {
                invoke("command_run_mcts", { searchN: mctsSearchN }).then(res => {
                    console.log(res);
                    let castRes = res as MctsScoreArray;
                    setMctsRows(castRes);
                });
            }, 100
        );
        setIntervalMcts(id);
    }

    function onClickStopMcts() {
        if (intervalMcts !== null) {
            clearInterval(intervalMcts);
            setIntervalMcts(null);
        }
    }

    return <div>
        {/* <Canv3d width={boardSize} height={boardSize} /> */}
        <div className="boardArea">

            <View3d width={boardSize} height={boardSize} board={board} rot={rot} focusIdx={focus}
                onPointerDown={onPointerDown} onPointerMove={onPointerMove} onPointerUp={onPointerUp} />
            <Square width={height - boardSize} height={height - boardSize} rot={rot} focusIdx={focus != null ? focus % 16 : null}
                setFocusIdx={onChangeFocus} onClickSquare={() => { onClickAction(focus) }} />
        </div>
        {message}
        <button onClick={onClickInit}>{'<<'}</button>
        <button onClick={onClickBack}>{'<'}</button>
        <button onClick={onClickNext}>{'>'}</button>
        <button onClick={onClickLast}>{'>>'}</button>
        <br />
        <div>
            <button onClick={onClickMate}>mate</button>
            <SearchRow row={mateRow} forcusIdx={focus != null ? focus % 16 : null} />
        </div>
        <div>
            <button onClick={onClickRunMcts}>eval</button>
            <button onClick={onClickStopMcts} >stop eval</button>
            {(() => {
                let rows = [];
                for (let i = 0; i < mctsRows?.length; i++) {
                    let row = mctsRows[i];
                    rows.push(<MctsRow onClick={() => { console.log("clicked board"); onClickAction(focus) }} setFocus={onChangeFocus} row={row} forcusIdx={focus != null ? focus % 16 : null} />);
                }
                return rows;
            })()}
        </div>
    </div>
}

export default Board