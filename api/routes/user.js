var restify = require('restify');


server.get('/echo/:name', function (req, res, next) {
    res.send(req.params);
    return next();
  });