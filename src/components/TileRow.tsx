import { Entity, Tile } from "rustlib";
import TileSpan from "./TileSpan";

type TileRowProps = {
	row: Tile[],
	tileIndex: number
	entities?: Entity[]
}

export default function TileRow(props: TileRowProps): React.JSX.Element {
	return (
		<>
			{
				props.row.map((i, index) => {
					/** @todo this is slow af */
					let entitiesForThisSpan = props.entities?.filter(i => i.location.x === index)
					return (
						<span
							key={"tile-" + i.location.x + "-" + i.location.y}
							id={"tile-" + i.location.x + "-" + i.location.y}
							style={{ "color": `#${(i.location.y + i.location.x) + 400}` }}
						>
							<TileSpan tile={i} entities={
                entitiesForThisSpan !== undefined && entitiesForThisSpan.length > 0 
                  ? entitiesForThisSpan 
                  : undefined
                } />
						</span>
					)
				})
			}
		</>
	)
}
