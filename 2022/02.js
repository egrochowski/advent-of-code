let games = require('./input/02').input

function cleanInput(input) {
	return input.trim().split('\n');
}

function determineWinner(game) {

	const [p1, p2] = game[0].split(' ')
	const p1Mapping = {
		'A': 1, 
		'B': 2,
		'C': 3
	};

	const p2Mapping = {
		'X': p1Mapping['A'],
		'Y': p1Mapping['B'],
		'Z': p1Mapping['C']
	};

	const winBonus  = 6;
	const drawBonus = 3
	const p1Points  = p1Mapping[p1];
	const p2Points  = p2Mapping[p2];

	const difference = p1Points - p2Points;
	// if there is no difference, the game is a draw
	if (!difference) {
		return drawBonus + p2Points;
	} else if ((p1Points > 0 && difference === 1) || difference === -2) {
		return p2Points;
	}
	return winBonus + p2Points;
}

function calcWins(games) {
	return games.reduce((sum, currentGame) => {
		return determineWinner(currentGame.split('\n')) + sum;
	}, 0)
}

function determineWinner2(game) {
	const [p1, p2] = game[0].split(' ');
	const winBonus  = 6;
	const drawBonus = 3;

	// if draw, just play what p1 played
	const mapping = {
		'A': 1, 
		'B': 2,
		'C': 3
	};

	// if draw
	const p1Points = mapping[p1];
	if (p2 === 'Y') {
		return mapping[p1] + drawBonus;
	} else if (p2 === 'X') {
		for (let option in mapping) {
			const difference = p1Points - mapping[option];
			if ((p1Points > 0 && difference === 1) || difference === -2) {
				return mapping[option];
			}
		}
	} else {
		for (let option in mapping) {
			const difference = mapping[option] - p1Points
			if (difference === 1 || difference === -2)
				return mapping[option] + winBonus
		}
	}
}

function calcWins2(games) {
	return games.reduce((sum, currentGame) => {
		return determineWinner2(currentGame.split('\n')) + sum;
	}, 0)
}

games = cleanInput(games);

console.log(calcWins(games)); // 12156
console.log(calcWins2(games));
