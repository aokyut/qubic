import "./SearchRow.css";

type MctsRowProps = {
    row: {
        action: number,
        score: number,
        q: number,
        na: number,
        n: number,
    },
    forcusIdx: number | null,
    setFocus: (idx: number) => void,
    onClick: () => void
}

const MctsRow = (props: MctsRowProps) => {
    return <div onClick={() => { props.onClick() }} onMouseMove={(_) => { props.setFocus(props.row.action) }} className={props.row.action === props.forcusIdx ? "search_row highlight" : "search_row"}>
        <p className="search_result">
            {props.row.action == -1 ? "None" : `action: ${props.row.action}, score: ${(100 * props.row.score).toFixed(1)}(${props.row.na}/${props.row.n}), q:${props.row.q.toFixed(3)}`}
        </p>
    </div>
}

export default MctsRow;