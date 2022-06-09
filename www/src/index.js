import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import {Bindings as tictactoe} from "tictactoe";
import {Bindings as mancala} from "mancala";
import {Bindings as reversi} from "reversi";
import {Bindings as connect4} from "connect4";

function rescale(arr,pondering) {
  var max = 0.0;
  var min = 1.0;
  var avg = 0.0;
  var scale = 1.0;
  
  if (!pondering) {
    return arr.map((item) => {
      let [i,w,s] = item;
      return [i,0,0];
    })
  }

  for (let [i,w,s] of arr) {
    avg += w;
    
    if (max < w) {
      max = w;
    }
    
    if (min > w) {
      min = w;
    }
  }
  
  avg /= arr.length;
  var good = max - avg;
  var bad = avg - min;
  if (good > bad) {
    scale = good;
  } else {
    scale = bad;
  }
  
  return arr.map((item) => {
    let [i,w,s] = item;
    if (w > avg) {
      return [i,(w - avg)/scale,0];
    } else if (w < avg) {
      return [i,0,(avg - w)/scale];
    } else {
      return [i,0,0];
    }
  });
}

function mix_color(r,g,b,good,bad) {
  let _r,_g,_b;
  if (good > 0) {
    _r = r*(1 - good);
    _g = g*(1 - good) + 255*good;
    _b = b*(1 - good);
  } else if (bad > 0) {
    _r = r*(1 - bad) + 255*bad;
    _g = g*(1 - bad);
    _b = b*(1 - bad);
  } else {
    _r = r;
    _g = g;
    _b = b;
  }
  
  return 'rgb('
    + String(_r) + ','
    + String(_g) + ','
    + String(_b) + ')';
}

/*------------------------------------------------------------------------------ 
 *                                TicTacToe  
 * ---------------------------------------------------------------------------*/

class TicTacToeBoard extends React.Component {
  renderSquare(i,good,bad) {
    let background = mix_color(100, 100, 100, good, bad);
    let color = 'white';
    var ch;
    switch(this.props.squares[i]) {
      case 'X': ch = 'X'; break;
      case 'O': ch = 'O'; break;
      default:  {
        ch = '-';
        color = background;
        break;
      }
    }

    return (
      <div
        className="tictactoe-cell" 
        key={i}
        onClick={() => this.props.onClick(i)}
        style={{
          backgroundColor:background,
          color:color,
          }}>
        {ch}
      </div>
    );
  }

  render() {
    var squares = [];
    let actions = rescale(this.props.actions, this.props.pondering);
    for (let i of [0,1,2,3,4,5,6,7,8]) {
      var item = item = [i,0,0];
      for (let [j,g,b] of actions) {
        if (i == j) {
          item = [i,g,b];
          break;
        }
      }
      squares.push(item);
    }
    
    return (
      <div className='board-container-parent'>
        <div className='board-container-child tictactoe-board'>
          {squares.map(([i,w,s]) => this.renderSquare(i,w,s))}
        </div>
      </div>
    );
  }
}

class TicTacToe extends React.Component {
  constructor(props) {
    super(props);
    this.q = 0.5;
    this.uiEnabled = true;
    this.game = tictactoe.new();
    this.pondering = false;
    this.game.ponder(10);
    this.state = JSON.parse(this.game.serialize());
  }

  ponder(i) {
    if (i <= 0) {
      
      var best;
      var max = 0;
      for (let a of this.state.actions) {
        let [i,w,_] = a;
        if (max <= w) {
          max = w;
          best = i;
        }
      }
      
      this.game.make(best);
      this.pondering = false;
      this.updateState();
      setTimeout(() => {
        this.uiEnabled = true;
      }, 100)

      return;
    }

    setTimeout(() => {
      this.pondering = true;
      this.game.ponder(50);
      this.updateState();
      this.ponder(i - 1)
    }, 50)
  }
  
  updateState() {
    let json = this.game.serialize();
    let update = JSON.parse(json);
    if ((update.info != null) && (update.side == 'O')) {
      this.q = update.info.q;
    }
    this.setState(update);
  }
  
  handleAI() {
    setTimeout(() => {
      if (this.state.result == null) {
        this.ponder(20);
      }
    },100)
  }

  handleClick(i) {
    if ((this.state.result != null) || !this.uiEnabled) {
      return;
    }

    this.uiEnabled = false;
    this.game.make(i);
    this.updateState();
    this.handleAI();
  }
  
  handleReset() {
    this.game = tictactoe.new();
    this.uiEnabled = true;
    this.updateState();
  }

  render() {
    let side  = this.state.side;
    let winner = this.state.side == 'X' ? 'O' : 'X';
    
    let status;
    let result = this.state.result;
    if (result != null) {
      if (result == "Draw")
      {
        status = "Draw";  
      }
      else
      {
        status = winner + " Wins!";
      }
    } else {
      status = side + "'s turn";
    }

    let q = (this.q*100).toFixed(0);

    return (
      <div className="game-layout">
        <div className="title">
          <div>Tic-Tac-Toe</div>
        </div>
        <div className="status">
          <div>{status}</div>
        </div>
        <div className="chance">
          <div>AI advantage: %{q}</div>
        </div>
        <div 
          className='reset'
          onClick={() => this.handleReset()}>
          reset
        </div>
        <TicTacToeBoard
            pondering={this.pondering}
            actions={this.state.actions}
            squares={this.state.board}
            onClick={i => this.handleClick(i)}
        />
      </div>
    );
  }
}

ReactDOM.createRoot(
  document.getElementById("tictactoe"),
).render(<TicTacToe/>);

/*------------------------------------------------------------------------------ 
 *                                Mancala  
 * ---------------------------------------------------------------------------*/

const mancala_ordering = [
  'R1','R2','R3','R4','R5','R6','RB',
  'L6','L5','L4','L3','L2','L1','LB'
];

class MancalaBoard extends React.Component {
  renderStones(i) {
    let stones = this.props.board[i];
    if (stones > 0) {
      return (
        <div className='stone-container-parent'>
          <div className='stone-container'>
            {
              [...Array(stones).keys()]
              .map((j) => <div className='stone' key={(i << 10) | j}></div>)
            }
          </div>
        </div>
      );
    }
  }

  renderPit(pit,i,good,bad) {
    let color = mix_color(100, 100, 100, good, bad);
    
    return (
      <div 
        className={"pit " + pit}
        key={pit}
        onClick={() => this.props.onClick(i)}
        style={{backgroundColor:color}}>
        {this.renderStones(i)}
      </div>
    );
  }

  render() {
    var pits = [];
    let actions = rescale(this.props.actions, this.props.pondering);
    for (let i = 0; i < mancala_ordering.length; i++) {
      let pit = mancala_ordering[i];
      var item = [pit,i,0,0];
      for (let [j,g,b] of actions) {
        if (i == j) {
          item = [pit,i,g,b];
          break;
        }
      }
      pits.push(item);
    }
    
    return (
      <div className='board-container-parent'>
        <div className='board-container-child mancala-board'>
          {
            pits.map((item) => {
              let [pit,i,good,bad] = item;
              return this.renderPit(pit,i,good,bad)
            })
          }
        </div>
      </div>
    );
  }
}

class Mancala extends React.Component {
  constructor(props) {
    super(props);
    this.q = 0.5;
    this.game = mancala.new();
    this.uiEnabled = true;
    this.game.ponder(10);
    this.state = JSON.parse(this.game.serialize());
  }
  
  updateState() {
    let json = this.game.serialize();
    let update = JSON.parse(json);
    if ((update.info != null) && (update.side == 'L')) {
      this.q = update.info.q;
    }
    this.setState(update);
    return update;
  }

  ponder(i) {
    if (i <= 0) {
      
      var best;
      var max = 0;
      
      for (let a of this.state.actions) {
        let [i,w,_] = a;
        if (max <= w) {
          max = w;
          best = i;
        }
      }
      
      this.game.make(best);
      this.pondering = false;
      let state = this.updateState();

      if (state.side == 'L') {
        this.handleAI();
      } else {
        setTimeout(() => {
          this.uiEnabled = true;
        }, 100)
      }

      return;
    }

    setTimeout(() => {
      this.pondering = true;
      this.game.ponder(50);
      this.updateState();
      this.ponder(i - 1)
    }, 50)
  }
  
  handleAI() {
    setTimeout(() => {
      if (this.state.result == null) {
        this.ponder(10);
      }
    },100)
  }
  
  handleClick(i) {
    if ((this.state.result != null) || !this.uiEnabled) {
      return;
    }
    
    if ((i < 0) || (i > 5)) {
      return;
    }

    this.game.make(i);
    let state = this.updateState();
    if (state.side == 'L') {
      this.uiEnabled = false;
      this.handleAI();
    }
  }
  
  handleReset() {
    this.game = mancala.new();
    this.uiEnabled = true;
    this.game.ponder(10);
    this.updateState();
  }

  render() {
    let side  = this.state.side == 'L' ? 'Left' : 'Right';
    let other = this.state.side == 'R' ? 'Left' : 'Right';
    
    let status;
    let result = this.state.result;
    if (result != null) {
      if (result == "Draw")
      {
        status = "Draw";  
      }
      else if (result == 'Win')
      {
        status = side + " Wins!";
      }
      else
      {
        status = other + " Wins!";
      }
    } else {
      status = side + "'s turn";
    }

    let q = (this.q*100).toFixed(0);

    return (
      <div className="game-layout">
        <div className="title">
          <div>Mancala</div>
        </div>
        <div className="status">
          <div>{status}</div>
        </div>
        <div className="chance">
          <div>AI advantage: %{q}</div>
        </div>
        <div 
          className='reset'
          onClick={() => this.handleReset()}>
          reset
        </div>
        <MancalaBoard
            pondering={this.pondering}
            board={this.state.board}
            actions={this.state.actions}
            onClick={(i) => this.handleClick(i)}
        />
      </div>
    );
  }
}

ReactDOM.createRoot(
  document.getElementById("mancala"),
).render(<Mancala/>);

/*------------------------------------------------------------------------------ 
 *                                Reversi  
 * ---------------------------------------------------------------------------*/

var reversi_ordering = [];
for (let u of ['7','6','5','4','3','2','1','0']) {
  for (let v of ['0','1','2','3','4','5','6','7']) {
    let oct = u + v;
    let i = parseInt(oct,8);
    reversi_ordering.push(i);
  }
}

class ReversiBoard extends React.Component {
  renderSpace(i,good,bad) {
    let color;
    
    let s = this.props.board[i];
    var w = s == 1;
    var b = s == 2;
    
    if (w) {
      color = "white";
    } else if (b) {
      color = "black";
    } else {
      color = mix_color(100, 100, 100, good, bad);
    }
    
    return (
      <div 
        className={"reversi-square " + color}
        key={i}
        onClick={() => this.props.onClick(i)}
        style={{backgroundColor:color}}>
        {}
      </div>
    );
  }

  render() {
    var spaces = [];
    let actions = rescale(this.props.actions, this.props.pondering);
    for (let i of reversi_ordering) {
      var item = [i,0,0];
      for (let [j,g,b] of actions) {
        if (i == j) {
          item = [i,g,b];
          break;
        }
      }
      spaces.push(item);
    }
    
    return (
      <div className='board-container-parent'>
        <div className='board-container-child reversi-board'>
          {
            spaces.map((item) => {
              let [i,good,bad] = item;
              return this.renderSpace(i,good,bad)
            })
          }
        </div>
      </div>
    );
  }
}

class Reversi extends React.Component {
  constructor(props) {
    super(props);
    this.q = 0.5;
    this.pondering = false;
    this.game = reversi.new();
    this.uiEnabled = true;
    this.game.ponder(10);
    this.state = JSON.parse(this.game.serialize());
  }
  
  updateState() {
    let json = this.game.serialize();
    let update = JSON.parse(json);
    if ((update.info != null) && (update.side == 'B')) {
      this.q = update.info.q;
    }
    this.setState(update);
    return update;
  }

  ponder(i) {
    if (i <= 0) {
      
      var best;
      var max = 0;
      
      for (let a of this.state.actions) {
        let [i,w,_] = a;
        if (max <= w) {
          max = w;
          best = i;
        }
      }

      this.game.make(best);
      this.game.ponder(10);
      this.pondering = false;
      let state = this.updateState();
      
      var pass = false;
      for (let a of state.actions) {
        let [i,w,s] = a;
        if (i >= 64) {
          pass = true;
        }
      }
      
      if (pass) {
        this.game.make(64);//pass
        this.handleAI();
      } else {
        setTimeout(() => {
          this.uiEnabled = true;
        }, 100)
      }

      return;
    }

    setTimeout(() => {
      this.pondering = true;
      this.game.ponder(50);
      this.updateState();
      this.ponder(i - 1)
    }, 50)
  }

  handleAI() {
    setTimeout(() => {
      if (this.state.result == null) {
        this.ponder(20);
      }
    },100)
  }

  handleClick(i) {
    if ((this.state.result != null) || !this.uiEnabled) {
      return;
    }

    this.game.make(i);
    let state = this.updateState();
    if (state.side == 'B') {
      this.uiEnabled = false;
      this.handleAI();
    }
  }
  
  handleReset() {
    this.game = reversi.new();
    this.uiEnabled = true;
    this.game.ponder(10);
    this.updateState();
  }

  render() {
    let side  = this.state.side == 'B' ? 'Black' : 'White';
    let other = this.state.side == 'W' ? 'Black' : 'White';
    
    let status;
    let result = this.state.result;
    if (result != null) {
      if (result == "Draw")
      {
        status = "Draw";  
      }
      else if (result == 'Win')
      {
        status = side + " Wins!";
      }
      else
      {
        status = other + " Wins!";
      }
    } else {
      status = side + "'s turn";
    }

    let q = (this.q*100).toFixed(0);

    return (
      <div className="game-layout">
        <div className="title">
          <div>Reversi</div>
        </div>
        <div className="status">
          <div>{status}</div>
        </div>
        <div className="chance">
          <div>AI advantage: %{q}</div>
        </div>
        <div
          className='reset'
          onClick={() => this.handleReset()}>
          reset
        </div>
        <ReversiBoard
            pondering={this.pondering}
            board={this.state.board}
            actions={this.state.actions}
            onClick={(i) => this.handleClick(i)}
        />
      </div>
    );
  }
}

ReactDOM.createRoot(
  document.getElementById("reversi"),
).render(<Reversi/>);

/*------------------------------------------------------------------------------ 
 *                                Connect4  
 * ---------------------------------------------------------------------------*/

var connect4_ordering = [];
for (let r of [5,4,3,2,1,0]) {
  for (let w of [0,1,2,3,4,5,6]) {
    connect4_ordering.push(w + r*7);
  }
}

class Connect4Board extends React.Component {
  renderSpace(i, good, bad) {
    let s = this.props.board[i];
    var w = s == 2;
    var b = s == 1;
    var color;
    
    if (w) {
      color = "white";
    } else if (b) {
      color = "black";
    } else {
      color = mix_color(100, 100, 100, good, bad);
    }
    
    return (
      <div 
        className={"connect4-square " + color}
        key={i}
        onClick={() => this.props.onClick(i)}
        style={{backgroundColor:color}}>
        {}
      </div>
    );
  }
  
  drop(c) {
    var i = c;
    while ((this.props.board[i] != 0) && (i < 35)) {
      i += 7;
    }
    
    return i;
  }
  
  render() {
    var spaces = [];
    let actions = rescale(this.props.actions, this.props.pondering);
    for (let i of connect4_ordering) {
      var item = [i,0,0];
      for (let [c,g,b] of actions) {
        let j = this.drop(c);
        if (i == j) {
          item = [i,g,b];
          break;
        }
      }
      spaces.push(item);
    }
    
    return (
      <div className='board-container-parent'>
        <div className='board-container-child connect4-board'>
          {
            spaces.map((item) => {
              let [i,good,bad] = item;
              return this.renderSpace(i,good,bad)
            })
          }
        </div>
      </div>
    );
  }
}

class Connect4 extends React.Component {
  constructor(props) {
    super(props);
    this.q = 0.5;
    this.pondering = false;
    this.game = connect4.new();
    this.uiEnabled = true;
    this.game.ponder(10);
    this.state = JSON.parse(this.game.serialize());
  }

  updateState() {
    let json = this.game.serialize();
    let update = JSON.parse(json);
    if ((update.info != null) && (update.side == 'B')) {
      this.q = update.info.q;
    }
    this.setState(update);
    return update;
  }

  ponder(i) {
    if (i <= 0) {
      
      var best;
      var max = 0;
      
      for (let a of this.state.actions) {
        let [i,w,_] = a;
        if (max <= w) {
          max = w;
          best = i;
        }
      }
      
      this.game.make(best);
      this.pondering = false;
      this.updateState();

      setTimeout(() => {
        this.uiEnabled = true;
      }, 100)

      return;
    }

    setTimeout(() => {
      this.pondering = true;
      this.game.ponder(50);
      this.updateState();
      this.ponder(i - 1)
    }, 50)
  }

  handleAI() {
    setTimeout(() => {
      if (this.state.result == null) {
        this.ponder(20);
      }
    },100)
  }
  
  handleClick(i) {
    let c = i % 7;
    if ((this.state.result != null) || !this.uiEnabled) {
      return;
    }

    this.game.make(c)
    this.updateState();
    this.uiEnabled = false;
    this.handleAI();
  }
  
  handleReset() {
    this.game = connect4.new();
    this.uiEnabled = true;
    this.game.ponder(10);
    this.updateState();
  }

  render() {
    let side  = this.state.side == 'B' ? 'Black' : 'White';
    let winner = this.state.side == 'W' ? 'Black' : 'White';
    
    let status;
    let result = this.state.result;
    if (result != null) {
      if (result == "Draw")
      {
        status = "Draw";  
      }
      else
      {
        status = winner + " Wins!";
      }
    } else {
      status = side + "'s turn";
    }
    
    let q = (this.q*100).toFixed(0);

    return (
      <div className="game-layout">
        <div className="title">
          <div>Connect Four</div>
        </div>
        <div className="status">
          <div>{status}</div>
        </div>
        <div className="chance">
          <div>AI advantage: %{q}</div>
        </div>
        <div 
          className='reset'
          onClick={() => this.handleReset()}>
          reset
        </div>
        <Connect4Board
            pondering={this.pondering}
            board={this.state.board}
            actions={this.state.actions}
            onClick={(i) => this.handleClick(i)}
        />
      </div>
    );
  }
}

ReactDOM.createRoot(
  document.getElementById("connect4"),
).render(<Connect4/>);
