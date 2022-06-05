import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import {TicTacToeBindings as ttt} from "tictactoe";
import {MancalaBindings as mb} from "mancala";
import {ReversiBindings as rb} from "reversi";
import {Connect4Bindings as c4b} from "connect4";

/*------------------------------------------------------------------------------ 
 *                                TicTacToe  
 * ---------------------------------------------------------------------------*/

class TicTacToeBoard extends React.Component {
  renderSquare(i) {
    return (
      <button 
        className="tictactoe-cell" 
        key={i}
        onClick={() => this.props.onClick(i)}>
        {this.props.squares[i]}
      </button>
    );
  }

  render() {
    return (
      <div className='tictactoe'>
        {[0,1,2,3,4,5,6,7,8].map((i) => this.renderSquare(i))}
      </div>
    );
  }
}

class TicTacToe extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      game: ttt.new(),
      uiEnabled:true
    };
  }
  
  handleClick(i) {
    let j = this.state.game.serialize();
    console.log(j)

    let game = JSON.parse(j);

    if ((game.result != null) || !this.state.uiEnabled) {
      return;
    }

    this.state.game.make(i)
    this.setState({uiEnabled:false})
    
    setTimeout(() => {
      let game = JSON.parse(this.state.game.serialize());
      if (game.result == null) {
        this.state.game.ai_make()
        this.setState(this.state)
        
        setTimeout(() => {
          this.setState({uiEnabled:true})
        }, 100)
      }
    }, 100);
  }
  
  handleReset() {
    this.setState({
      game: ttt.new(),
      uiEnabled: true,
    })
  }

  render() {
    let game = JSON.parse(this.state.game.serialize())
    
    let status;
    if (game.result != null) {
      if (game.result == "Draw")
      {
        status = "Draw";
      }
      else
      {
        status = "Winner: " + (game.side == 'X' ? 'O' : 'X');
      }
    } else {
      status = "Next player: " + game.side;
    }
    
    return (
      <div className="tictactoe-grid">
        <div className="tictactoe-grid-style tictactoe-grid-board">
          <TicTacToeBoard
            squares={game.board}
            onClick={i => this.handleClick(i)}
          />
        </div>
        <div className="tictactoe-grid-style tictactoe-grid-status">
          <div className="game-info">
            <div>{status}</div>
          </div>
        </div>
        <div className="tictactoe-grid-style tictactoe-grid-buton">
          <button className="tictactoe-reset" onClick={() => this.handleReset()}>
            {"reset"}
          </button>
        </div>
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

class MancalaBoard extends React.Component {
  renderPit(pit,i) {
    return (
      <button 
        className={"pit " + pit}
        key={pit}
        onClick={() => this.props.onClick(i)}>
        {this.props.pit[i]}
      </button>
    );
  }

  render() {
    return (
      <div className='mancala-board'>
        {
          [
            'R1','R2','R3','R4','R5','R6','RB',
            'L6','L5','L4','L3','L2','L1','LB'
          ]
          .map((pit,i) => this.renderPit(pit,i))
        }
      </div>
    );
  }
}

class Mancala extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      game: mb.new(),
      uiEnabled:true
    };
  }
  
  getGame() {
    let j = this.state.game.serialize();
    //console.log(j)
    return JSON.parse(j)
  }
  
  handleAI() {
    setTimeout(() => {
      let game = this.getGame();
      
      if ((game.result == null) && (game.side == 'L')) {
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
    
    if ((i < 0) || (i > 5)) {
      return;
    }

    this.state.game.make(i)
    this.setState({uiEnabled:false})
    this.handleAI();
  }
  
  handleReset() {
    this.setState({
      game: mb.new(),
      uiEnabled: true,
    });
  }

  render() {
    let game = this.getGame();
    
    let status;
    if (game.result != null) {
      if (game.result == "Draw")
      {
        status = "Draw";
      }
      else
      {
        status = "Winner: " + (game.side == 'L' ? 'R' : 'L');
      }
    } else {
      status = "Next player: " + game.side;
    }

    return (
      <div className='mancala'>
        <MancalaBoard
            pit={game.pit}
            onClick={i => this.handleClick(i)}
        />
        <div className='mancala-status'>
          {status}
        </div>
        <div 
          className='mancala-reset'
          onClick={() => this.handleReset()}>
          reset
        </div>
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
      <div className='reversi-board-square'>
        <div className='reversi-board'>
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
      game: rb.new(),
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
      game: rb.new(),
      uiEnabled: true,
    });
  }

  render() {
    let game = this.getGame();
    
    let status;
    if (game.result != null) {
      if (game.result == "Draw")
      {
        status = "Draw";
      }
      else
      {
        status = "Winner: " + (game.side == 'W' ? 'B' : 'W');
      }
    } else {
      status = "Next player: " + game.side;
    }

    return (
      <div className='reversi'>
        <ReversiBoard
            pit={game.pit}
            onClick={i => this.handleClick(i)}
            board={game.board}
        />
        <div className='reversi-status'>
          {status}
        </div>
        <div 
          className='reversi-reset'
          onClick={() => this.handleReset()}>
          reset
        </div>
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
      <div className='connect4-board-square'>
        <div className='connect4-board'>
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
      game: c4b.new(),
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
      game: c4b.new(),
      uiEnabled: true,
    });
  }

  render() {
    let game = this.getGame();
    
    let status;
    if (game.result != null) {
      if (game.result == "Draw")
      {
        status = "Draw";
      }
      else
      {
        status = "Winner: " + (game.side == 'Y' ? 'R' : 'Y');
      }
    } else {
      status = "Next player: " + game.side;
    }

    return (
      <div className='connect4'>
        <Connect4Board
            onClick={i => this.handleClick(i)}
            board={game.board}
        />
        <div className='connect4-status'>
          {status}
        </div>
        <div 
          className='connect4-reset'
          onClick={() => this.handleReset()}>
          reset
        </div>
      </div>
    );
  }
}

ReactDOM.createRoot(
  document.getElementById("connect4"),
).render(<Connect4/>);
