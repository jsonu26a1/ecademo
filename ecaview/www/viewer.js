class Row {
  constructor(offset, data) {
    this.offset = offset;
    this.data = data;
  }
  /*
    example:
    - - - - - - - - - -
              |> offset
      |> x
  */
  slice(x, len) {
    let slice = [];
    if(x < this.offset) {
      let start = this.offset - x;
      let head = (this.data[0] & 0b1000_0000) != 0 | 0;
      // for(let i = 0; i < start; i++) {
      //   slice.push(head);
      // }
      slice.length = start;
      slice.fill(head, 0, start);
    }
    if(x + len < this.offset) {
      for(let b of this.data) {
        if(slice.length > len) {
          slice.length = len;
          break;
        }
        for(let i = 7; i >= 0; i--) {
          slice.push((b >> i) & 1);
        }
      }
    }
    if(slice.length < len) {
      let tail = this.data[this.data.length-1] & 1;
      let start = slice.length
      slice.length = len;
      slice.fill(tail, start, len);
    }
    return slice;
  }
}

function import_row(buffer, byteOffset) {
  let dv = new DataView(buffer, byteOffset);
  let offset = Number(dv.getBigInt64(0));
  let len = Number(dv.getBigUint64(8));
  let data = new Uint8Array(buffer, byteOffset + 16, len);
  let row = new Row(offset, data);
  return [row, byteOffset + 16 + len];
}

function import_automaton(buffer) {
  let rows = [];
  let byteOffset = 0;
  while(byteOffset < buffer.byteLength) {
    let [row, new_offset] = import_row(buffer, byteOffset);
    rows.push(row);
    byteOffset = new_offset;
  }
  return rows;
}

class Automaton {
  constructor(rows) {
    this.rows = rows;
  }
  render(x, y, w, h) {
    let canvas = document.createElement("canvas");
    canvas.width = w;
    canvas.height = h;
    let ctx = canvas.getContext("2d");
    ctx.fillStyle = "black";
    for(let py = y; py < y + h; py++) {
      let row = this.rows[py];
      if(!row) continue;
      let slice = row.slice(x, w);
      for(let i = 0; i < w; i++) {
        if(slice[i] == 0) {
          ctx.fillRect(i, py - y, 1, 1);
        }
      }
    }
    return canvas;
  }
}
