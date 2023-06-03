import { Tile } from "rustlib";

type TileSpanProps = {
  tile: Tile,
}

export default function TileSpan(props: TileSpanProps): React.JSX.Element {
  return (
    <>
      {props.tile.char}
    </>
  )
}
