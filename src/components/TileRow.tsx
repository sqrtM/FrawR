import { Tile } from "rustlib";
import TileSpan from "./TileSpan";

type TileRowProps = {
	row: Tile[],
}

export default function TileRow(props: TileRowProps): React.JSX.Element {
	return (
		<>
			{
				props.row.map((i) => {
					return (
						<span
							key={"tile-" + i.location.x + "-" + i.location.y}
							id={"tile-" + i.location.x + "-" + i.location.y}
							style={{ "color": `#${(i.location.y + i.location.x) + 400}` }}
						>
							<TileSpan tile={i} />
						</span>
					)
				})
			}
		</>
	)
}