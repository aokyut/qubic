import { Canvas, useFrame } from '@react-three/fiber';
import { DirectionalLight, Mesh, Vector3, Color, Event } from 'three';
import { Plane } from '@react-three/drei';
import { useState, useEffect, useRef } from "react";

const dis = 0.9;

type RigProps = {
    rad: number
}

const Rig = (props: RigProps) => {
    // const [rad, setRad] = useState(0);
    return useFrame((state) => {
        // state.camera.position.lerp(v.set(state.mouse.x / 2, state.mouse.y / 2, 10), 0.05)
        // setRad(rad + 0.01);
        state.camera.position.y = dis * 4;
        state.camera.position.x = dis * 10 * Math.sin(props.rad);
        state.camera.position.z = dis * 10 * Math.cos(props.rad);
        state.camera.lookAt(0, -1, 0);
    })
};

const Black = "#333333";
const White = "white";
const Green = "green";

type BallProps = {
    position: [x: number, y: number, z: number];
    color: string;
};

const Ball: React.FC<BallProps> = (props) => {
    const mesh = useRef<Mesh>(null!);
    useFrame(() => (mesh.current.rotation.x += 0.01));

    return (
        <mesh
            position={props.position}
            ref={mesh}
            scale={1}
        >
            {/* <boxGeometry args={[1, 1, 1]} /> */}
            <sphereGeometry args={[0.7, 16, 8]} />
            <meshStandardMaterial color={props.color} />
        </mesh>
    );
};

type BallsProps = {
    ball: string,
    focusIdx: null | number
}

function Balls(props: BallsProps) {
    let list = []
    for (let i = 0; i < 64; i++) {
        const x = i % 4;
        const y = Math.floor(i / 16);
        const z = Math.floor((i - y * 16) / 4);
        if (props.ball[i] == "O") {
            list.push(<Ball position={[x * 2 - 3, y * 2 - 3, z * 2 - 3]} color={Black} />)
        } else if (props.ball[i] == "X") {
            list.push(<Ball position={[x * 2 - 3, y * 2 - 3, z * 2 - 3]} color={White} />)
        }
    }
    if (props.focusIdx === null) return list;
    const x = props.focusIdx % 4;
    const y = Math.floor(props.focusIdx / 16);
    const z = Math.floor((props.focusIdx - y * 16) / 4);
    list.push(<Ball position={[x * 2 - 3, y * 2 - 3, z * 2 - 3]} color={Green} />)
    return list;
}

type CanvasProps = {
    width: number,
    height: number,
    board: string,
    rot: number,
    focusIdx: null | number,
    onPointerDown: React.PointerEventHandler<HTMLDivElement>,
    onPointerUp: React.PointerEventHandler<HTMLDivElement>,
    onPointerMove: React.PointerEventHandler<HTMLDivElement>
}

function View3d(props: CanvasProps) {
    return <div style={{ width: props.width, height: props.height }}>
        <Canvas onPointerDown={props.onPointerDown}
            onPointerUp={props.onPointerUp}
            onPointerMove={props.onPointerMove}>
            <ambientLight intensity={1} />
            <directionalLight intensity={3} position={[10, 10, 10]} />
            <Rig rad={props.rot} />
            <pointLight position={[-10, -10, -10]} />
            <Balls ball={props.board} focusIdx={props.focusIdx} />
            <Plane position={[0, -3.8, 0]} rotation={[-Math.PI / 2, 0, 0]} args={[10, 10]} receiveShadow>
                <meshStandardMaterial color="#f55" />
            </Plane>
            {/* <scene background={new Color(0, 0, 255)} /> */}
        </Canvas>
    </div>
}

export default View3d;