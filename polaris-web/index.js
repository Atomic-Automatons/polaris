function renderWayPoint(ctx, point, opts) {
	opts = opts || {};
	var size = opts.size || 10;
	var sizeOffset = size / 2;

	var pointX = point.x - sizeOffset;
	var pointY = ctx.canvas.height - (point.y + sizeOffset);

	ctx.fillRect(pointX, pointY, size, size);

	if (opts.text) {
		ctx.fillText(opts.text, point.x, pointY + 2 * size);
	}
}

function renderRadius(ctx, x, y, radius){
	ctx.beginPath();
	ctx.arc(x, ctx.canvas.height - y, radius, radius, Math.PI * 2, true);
	ctx.closePath();
	ctx.stroke();
}

function renderPath(ctx, path, obstacles, radius, opts) {
	opts = opts || {};
	
	ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
	ctx.fillStyle = "black";
	ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);
	ctx.fillStyle = "white";
	for (var i = 0; i != path.length; i++) {
		renderWayPoint(ctx, path[i], {
			text: (i + 1),
			size: 10,
		});
		ctx.strokeStyle = "white";
		if (opts.wayPoints.renderRadius) {
			renderRadius(ctx, path[i].x, path[i].y, radius);
		}
	}
	ctx.fillStyle = "orange";
	for (var i = 0; i != obstacles.length; i++) {
		var point = obstacles[i];
		var size = 10;
		var sizeOffset = size / 2;
		var pointY = ctx.canvas.height - (point.y + sizeOffset);
		ctx.fillRect(point.x - sizeOffset, pointY, size, size);
		
		ctx.strokeStyle = "orange";
		if (opts.obstacles.renderRadius) {
			renderRadius(ctx, point.x, point.y, radius);
		}
	}
}

import("./crate/pkg").then(Polaris => {
	let start = {
		x: 25,
		y: 50,
	};

	let end = {
			x: 600, 
			y: 400
	};
	
	let radius = 50;

	let map = new Polaris.Map(new Polaris.Point(start.x, start.y), new Polaris.Point(end.x, end.y), radius);
	//map.set_limit(300);
	let obstacles = [{
			x: 450.0,
			y: 300.0
		},
		{
			x: 400.0,
			y: 350.0
		},
		{
			x: 281,
			y: 205,
		},
		{
			x: 300,
			y: 350,
		},
	];

	for (var i = 0; i != obstacles.length; i++) {
		map.add_obstacle(new Polaris.Point(obstacles[i].x, obstacles[i].y));
	}

	let startTime = performance.now();
	let path = map.compile();
	let endTime = performance.now();
	console.log(path, endTime - startTime);

	let canvas = document.getElementById('canvas');
	let renderObstacleRadius = document.getElementById('obstacle-radius-checkbox');
	let renderWayPointRadius = document.getElementById('waypoint-radius-checkbox');
	let ctx = canvas.getContext('2d');
	let renderOptions = {
		obstacles: {
			renderRadius: renderObstacleRadius.checked,
		},
		wayPoints: {
			renderRadius: renderWayPointRadius.checked,
		}
	};

	
	let points = document.getElementById('points');
	renderObstacleRadius.onclick = function (e) {
		renderOptions.obstacles.renderRadius = e.target.checked;
		renderPath(ctx, path, obstacles, radius, renderOptions);
	}
	
	renderWayPointRadius.onclick = function(e){
			renderOptions.wayPoints.renderRadius = e.target.checked;
			renderPath(ctx, path, obstacles, radius, renderOptions);
	}
	
	points.innerHTML = '';
	for(var i = 0; i != path.length; i++){
		points.innerHTML += '<div>Point ' + (i + 1) + ': ('+ path[i].x + ', ' + path[i].y + ')</div>';
	}

	renderPath(ctx, path, obstacles, radius, renderOptions);
	
	window.addEventListener('click', function(e){
		if(e.target == canvas){
			var rect = canvas.getBoundingClientRect();
			var x = e.clientX - rect.left;
			var y = canvas.height - (e.clientY - rect.top);
			obstacles.push({x, y});
			map.add_obstacle(new Polaris.Point(x, y));
			path = map.compile();
			renderPath(ctx, path, obstacles, radius, renderOptions);
		}
	});
});
