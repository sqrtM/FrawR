import { Entity, Tile, World } from "rustlib";
import TileRow from "./TileRow";

type TileMapProps = {
  world: World,
  width: number
}

export default function TileMap(props: TileMapProps): React.JSX.Element {

  // this can probably be done faster... Rust?
  const splitIntoRows = (arr: Tile[]): Tile[][] => {
    let row: Tile[][] = [];
    for (let i = 0; i < arr.length && arr.length > 0; i += props.width) {
      row.push(arr.slice(i, i + props.width))
    }
    return row;
  }

  return (
    <>
      {
        splitIntoRows(props.world.get_tiles()).map((i, index) => {
          /** @todo this is slow af */
          let entitiesForThisRow = (props.world.get_entities() as Entity[]).filter(i => i.location.y === index);
          return (
            <div key={"row-" + index} id={"row-" + index}>
              <TileRow row={i} tileIndex={index} entities={entitiesForThisRow.length > 0 ? entitiesForThisRow : undefined} />
            </div>
          )
        })
      }
    </>
  )
}
