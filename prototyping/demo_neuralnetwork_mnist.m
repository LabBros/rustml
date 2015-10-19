1;
more off;

% load the dataset if not already loaded
if exist("trainX") == 0
	display("loading dataset...");
	load("data/mnist.txt.zip");
end

% plot some digits
clf;
img = zeros(280, 280);
for y = 0:9
	for x = 0:9
		v = trainX(y * 10 + x + 1, :);
		m = reshape(v, 28, 28)';
		img( (y * 28 + 1):(y * 28 + 28), (x * 28 + 1):(x * 28 + 28) ) = m;
	end
end
imagesc(img);

display('press enter to continue...');
pause;


display("training a neural network...");

n = 1000;
[yv, mapping] = convert(trainY);
[p1, p2, err] = nn_train(28 * 28, 30, size(mapping, 1), trainX(1:n, :), yv(1:n, :));

err(size(err, 1))
display("learning curve...");

clf;
plot(err, 'linewidth', 2);
grid on;
title("Learning curve");

display('press enter to continue...');
pause;

% TODO
c = 0;
s = 0;
for i = 1:100
	v = nn_predict(p1, p2, testX(i, :))';
	[mxval, mxidx] = max(v);
	r = [testY(i, :), mxidx];
	r
	if testY(i, 1) == mxidx
		c += 1;
	end
	s += 1;
end
c / s
