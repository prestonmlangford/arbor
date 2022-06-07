import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import {Bindings as tictactoe} from "tictactoe";
import {Bindings as mancala} from "mancala";
import {ReversiBindings as reversi} from "reversi";
import {Connect4Bindings as connect4} from "connect4";

/*------------------------------------------------------------------------------ 
 *                                TicTacToe  
 * ---------------------------------------------------------------------------*/

class TicTacToeBoard extends React.Component {
  renderSquare(i,w,s) {
    let r = 255*w;
    let g = 255 - r;
    let b = g;
    var color = 
      'rgba('
      + String(255) + ','
      + String(g) + ','
      + String(b) + ','
      + String(1) + ')';
    
    var ch;
    switch(this.props.squares[i]) {
      case 'X': ch = 'X'; break;
      case 'O': ch = 'O'; break;
      default:  ch = '_'; break;
    }

    return (
      <button 
        className="tictactoe-cell" 
        key={i}
        onClick={() => this.props.onClick(i)}
        style={{backgroundColor:color}}>
        {ch}
      </button>
    );
  }

  render() {
    var squares = [];
    for(let i of [0,1,2,3,4,5,6,7,8]) {
      var item = [i,0,0];
      if (this.props.pondering) {
        for (let [j,w,s] of this.props.actions) {
          if (i == j) {
            item = [i,w,1];
            break;
          }
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
        if (max < w) {
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
    console.log(json);
    let update = JSON.parse(json);
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
    let status;
    if (this.state.result != null) {
      if (this.state.result == "Draw")
      {
        status = "Draw";
      }
      else
      {
        status = "Winner: " + (this.state.side == 'X' ? 'O' : 'X');
      }
    } else {
      status = "Next player: " + this.state.side;
    }
    
    return (
      <div className="game-layout">
        <div className="title">
          <div>Tic-Tac-Toe</div>
        </div>
        <div className="status">
          <div>{status}</div>
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
  renderPit(pit,i,good,bad) {
    let r,g,b;
    if (good > 0) {
      r = 255*(1 - good);
      g = 255;
      b = r;
    } else if (bad > 0) {
      r = 255
      g = 255*(1 - bad);
      b = g;
    } else {
      r = 255;
      g = 255;
      b = 255;
    }

    var color = 
      'rgb('
      + String(r) + ','
      + String(g) + ','
      + String(b) + ')';
    
    return (
      <button 
        className={"pit " + pit}
        key={pit}
        onClick={() => this.props.onClick(i)}
        style={{backgroundColor:color}}>
        {
          //pit
          this.props.board[i]
        }
      </button>
    );
  }

  render() {
    var pits = [];
    var max = 0.0;
    var min = 1.0;
    var avg = 0.0;
    var scale = 1.0;
    if (this.props.pondering) {
      for (let [i,w,s] of this.props.actions) {
        avg += w;
        
        if (max < w) {
          max = w;
        }
        
        if (min > w) {
          min = w;
        }
      }
      avg /= this.props.actions.length;
      var good = max - avg;
      var bad = avg - min;
      if (good > bad) {
        scale = good;
      } else {
        scale = bad;
      }
    }
    
    for(let i = 0; i < mancala_ordering.length; i++) {
      let name = mancala_ordering[i];
      var item = [name,i,0,0];
      if (this.props.pondering) {
        for (let [j,w,s] of this.props.actions) {
          if (i == j) {
            if (w > avg) {
              item = [name,i,(w - avg)/scale,0];
            } else if (w < avg) {
              item = [name,i,0,(avg - w)/scale];
            } else {
              item = [name,i,0,0];
            }
            
            break;
          }
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
    this.game = mancala.new();
    this.uiEnabled = true;
    this.game.ponder(10);
    this.state = JSON.parse(this.game.serialize());
  }
  
  updateState() {
    let json = this.game.serialize();
    let update = JSON.parse(json);
    this.setState(update);
    return update;
  }
  
  
  ponder(i) {
    if (i <= 0) {
      
      var best;
      var max = 0;
      
      
      console.log("----- Move list -----");
      for (let a of this.state.actions) {
        let [i,w,_] = a;
        console.log("i: %d, w: %f",i,w);
        if (max < w) {
          max = w;
          best = i;
        }
      }
      console.log("---------------------");
      
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
    let winner = this.state.side == 'L' ? 'Right' : 'Left';
    let side   = this.state.side == 'L' ? 'Left'  : 'Right';
    let status;
    if (this.state.result != null) {
      if (this.state.result == "Draw")
      {
        status = "Draw";
      }
      else
      {
        status = "Winner: " + winner;
      }
    } else {
      status = "Next player: " + side
    }

    return (
      <div className="game-layout">
        <div className="title">
          <div>Mancala</div>
        </div>
        <div className="status">
          <div>{status}</div>
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

var reversi_board_ordering = [];
for (let u of ['7','6','5','4','3','2','1','0']) {
  for (let v of ['0','1','2','3','4','5','6','7']) {
    let oct = u + v;
    let i = parseInt(oct,8);
    reversi_board_ordering.push(i);
  }
}

class ReversiBoard extends React.Component {
  renderSpace(i) {
    let s = this.props.board[i];
    var w = s == 1;
    var b = s == 2;
    var color = "grey";
    
    if (w) {
      color = "white";
    }
    
    if (b) {
      color = "black";
    }
    
    return (
      <button 
        className={"reversi-square " + color}
        key={i}
        onClick={() => this.props.onClick(i)}>
        {}
      </button>
    );
  }

  render() {
    return (
      <div className='board-container-parent'>
        <div className='board-container-child reversi-board'>
          {
            reversi_board_ordering
            .map((i) => this.renderSpace(i))
          }
        </div>
      </div>
    );
  }
}

class Reversi extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      game: reversi.new(),
      uiEnabled:true
    };
  }
  
  getGame() {
    let j = this.state.game.serialize();
    return JSON.parse(j)
  }
  
  handleAI() {
    setTimeout(() => {
      let game = this.getGame();
      
      if ((game.result == null) && (game.side == 'B')) {
        this.state.game.ai_make()
        this.setState(this.state)
        
        setTimeout(() => {
          this.handleAI()
        }, 100)
      } else {
        this.setState({uiEnabled: true})
      }
    }, 100);
  }
  
  handleClick(i) {
    let game = this.getGame();

    if ((game.result != null) || !this.state.uiEnabled) {
      return;
    }

    this.state.game.make(i)
    this.setState({uiEnabled:false})
    this.handleAI();
  }
  
  handleReset() {
    this.setState({
      game: reversi.new(),
      uiEnabled: true,
    });
  }

  render() {
    let game = this.getGame();
    
    let winner = game.side == 'W' ? 'Red'     : 'Yellow';
    let side   = game.side == 'W' ? 'Yellow'  : 'Red';
    
    let status;
    if (game.result != null) {
      if (game.result == "Draw")
      {
        status = "Draw";
      }
      else
      {
        status = "Winner: " + winner;
      }
    } else {
      status = "Next player: " + side;
    }

    return (
      <div className="game-layout">
        <div className="title">
          <div>Reversi</div>
        </div>
        <div className="status">
          <div>{status}</div>
        </div>
        <div 
          className='reset'
          onClick={() => this.handleReset()}>
          reset
        </div>
        <ReversiBoard
            pit={game.pit}
            onClick={i => this.handleClick(i)}
            board={game.board}
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

var connect4_board_ordering = [];
for (let r of [5,4,3,2,1,0]) {
  for (let w of [0,1,2,3,4,5,6]) {
    connect4_board_ordering.push(w + r*7);
  }
}

class Connect4Board extends React.Component {
  renderSpace(i) {
    let s = this.props.board[i];
    var y = s == 1;
    var r = s == 2;
    var color = "grey";
    
    if (y) {
      color = "yellow";
    }
    
    if (r) {
      color = "red";
    }
    
    return (
      <button 
        className={"connect4-square " + color}
        key={i}
        onClick={() => this.props.onClick(i)}>
        {}
      </button>
    );
  }

  render() {
    return (
      <div className='board-container-parent'>
        <div className='board-container-child connect4-board'>
          {
            connect4_board_ordering
            .map((i) => this.renderSpace(i))
          }
        </div>
      </div>
    );
  }
}

class Connect4 extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      game: connect4.new(),
      uiEnabled:true
    };
  }
  
  getGame() {
    let j = this.state.game.serialize();
    console.log(j)
    return JSON.parse(j)
  }
  
  handleAI() {
    setTimeout(() => {
      let game = this.getGame();
      
      if ((game.result == null) && (game.side == 'Y')) {
        this.state.game.ai_make()
        this.setState(this.state)
        
        setTimeout(() => {
          this.handleAI()
        }, 100)
      } else {
        this.setState({uiEnabled: true})
      }
    }, 100);
  }
  
  handleClick(i) {
    let game = this.getGame();
    let c = i % 7;
    if ((game.result != null) || !this.state.uiEnabled) {
      return;
    }

    this.state.game.make(c)
    this.setState({uiEnabled:false})
    this.handleAI();
  }
  
  handleReset() {
    this.setState({
      game: connect4.new(),
      uiEnabled: true,
    });
  }

  render() {
    let game = this.getGame();
    
    let winner = game.side == 'Y' ? 'Red'     : 'Yellow';
    let side   = game.side == 'Y' ? 'Yellow'  : 'Red';
    
    let status;
    if (game.result != null) {
      if (game.result == "Draw")
      {
        status = "Draw";
      }
      else
      {
        status = "Winner: " + winner;
      }
    } else {
      status = "Next player: " + side;
    }

    return (
      <div className="game-layout">
        <div className="title">
          <div>Connect Four</div>
        </div>
        <div className="status">
          <div>{status}</div>
        </div>
        <div 
          className='reset'
          onClick={() => this.handleReset()}>
          reset
        </div>
        <Connect4Board
            onClick={i => this.handleClick(i)}
            board={game.board}
        />
      </div>
    );
  }
}

ReactDOM.createRoot(
  document.getElementById("connect4"),
).render(<Connect4/>);
