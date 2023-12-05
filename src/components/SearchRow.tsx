import "./SearchRow.css";

type SearchRowProps = {
    row: { depth: number, action: number },
    forcusIdx: number | null,
}

const SearchRow = (props: SearchRowProps) => {
    return <div className={props.row.action === props.forcusIdx ? "search_row highlight" : "search_row"}>
        <p className="search_result">
            {props.row.action == -1 ? "None" : `depth: ${props.row.depth}, action: ${props.row.action}`}
        </p>
    </div>
}

export default SearchRow;