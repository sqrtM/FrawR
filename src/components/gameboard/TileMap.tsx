import { Entity, Point, Tile, } from '../../../rustlib/pkg';
import TileRow from "./TileRow";

type TileMapProps = {
  tiles: [Point, Tile][],
  entities: Entity[],
  width: number
  player: Entity
}

export default function TileMap(props: TileMapProps): React.JSX.Element {
  // this can probably be done faster... Rust?
  const splitIntoRows = (arr: [Point, Tile][]): [Point, Tile][][] => {
    const row: [Point, Tile][][] = [];
    for (let i = 0; i < arr.length && arr.length > 0; i += props.width) {
      row.push(arr.slice(i, i + props.width))
    }
    return row;
  }

  return (
    <>
      {
        splitIntoRows(props.tiles).map((i, index) => {
          const entitiesForThisRow = props.entities.filter(i => i.location.y === index);
          const player = props.player.location.y === index ? props.player : false;
          return (
            <div key={"row-" + index} id={"row-" + index}>
              <TileRow row={i} tileIndex={index} player={player} entity={entitiesForThisRow} />
            </div>
          )
        })
      }
    </>
  )
}
