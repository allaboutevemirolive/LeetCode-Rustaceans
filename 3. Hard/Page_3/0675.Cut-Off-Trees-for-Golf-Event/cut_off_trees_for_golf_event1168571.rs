// https://leetcode.com/problems/cut-off-trees-for-golf-event/solutions/1168571/c-bfs-easy-to-read/
public class Solution {

	private IList<IList<int>> _forest;

	public int CutOffTree(IList<IList<int>> forest) {
		_forest = forest;

		//build the heights sorted dictionary
		var heights = new SortedDictionary<int, Tuple<int,int>>();
		var heightsQueue = new Queue<Tuple<int,int>>();

		for (var i = 0; i < forest.Count; i++) {
			for (var j = 0; j < forest[i].Count; j++) {
				var height = forest[i][j];
				if (height > 0) {
					heights.Add(height, Tuple.Create(i, j));
				}
			}
		}

		foreach (var pair in heights) {
			heightsQueue.Enqueue(pair.Value);
		}

		var current = Tuple.Create(0, 0);

		var totalDistance = 0;
		try {
			while (heightsQueue.Count > 0) {
				var destination = heightsQueue.Dequeue();
				if (current.Item1 == destination.Item1 && current.Item2 == destination.Item2) {
					current = destination;
					continue;
				}

				var distance = getDistance(current.Item1, current.Item2, destination.Item1, destination.Item2);
				current = destination;
				totalDistance += distance;
			}

			return totalDistance;
		}
		catch (Exception e) {
			return -1;
		}
	}

	int getDistance(int iSource, int jSource, int iDest, int jDest) {
		var q = new Queue<Tuple<int,int>>();
		var source = Tuple.Create(iSource, jSource);
		q.Enqueue(source);

		var distance = -1;
		bool[,] visitedArray = new bool[_forest.Count, _forest[0].Count];
		visitedArray[iSource, jSource] = true;

		var neighbors = new List<Tuple<int, int>>();
		while (q.Count > 0) {
			neighbors.Clear();
			distance++;
			while (q.Count > 0) { // process the row
				var current = q.Dequeue();
				if (current.Item1 == iDest && current.Item2 == jDest) {
					return distance;
				}

				//get neighbors
				//above
				if (current.Item1 > 0) {
					var above = Tuple.Create(current.Item1 - 1, current.Item2);
					var height = _forest[above.Item1][above.Item2];
					if (height != 0 && !visitedArray[above.Item1, above.Item2]) {
						neighbors.Add(above);
						visitedArray[above.Item1, above.Item2] = true;
					}
				}

				//below
				if (current.Item1 < _forest.Count - 1) {
					var below = Tuple.Create(current.Item1 + 1, current.Item2);
					var height = _forest[below.Item1][below.Item2];
					if (height != 0 && !visitedArray[below.Item1, below.Item2]) {
						neighbors.Add(below);
						visitedArray[below.Item1, below.Item2] = true;
					}
				}

				//left
				if (current.Item2 > 0) {
					var left = Tuple.Create(current.Item1, current.Item2 - 1);
					var height = _forest[left.Item1][left.Item2];
					if (height != 0 && !visitedArray[left.Item1, left.Item2]) {
						neighbors.Add(left);
						visitedArray[left.Item1, left.Item2] = true;
					}
				}

				//right
				if (current.Item2 < _forest[0].Count - 1) {
					var right = Tuple.Create(current.Item1, current.Item2 + 1);
					var height = _forest[right.Item1][right.Item2];
					if (height != 0 && !visitedArray[right.Item1, right.Item2]) {
						neighbors.Add(right);
						visitedArray[right.Item1, right.Item2] = true;
					}
				}
			} //finished the row

			//add the items to the queue
			foreach (var item in neighbors) {
				q.Enqueue(item);
			}
		}

		throw new Exception("we couldn't find a path to the next tree");
	}

}