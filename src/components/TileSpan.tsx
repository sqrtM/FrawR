import { Entity, Tile } from "rustlib";

type TileSpanProps = {
  tile: Tile,
  entities?: Entity[]
}

export default function TileSpan(props: TileSpanProps): React.JSX.Element {
  if (props.entities) {
    console.log(props.entities)
  }
  return (
    <>
      {
      props.entities !== undefined 
        ? props.entities[0].char 
        : props.tile.char
      }
    </>
  )
}
