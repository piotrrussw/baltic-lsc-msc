const express = require('express'),
    request = require('request'),
    bodyParser = require('body-parser'),
    https = require('https'),
    cache = require('memory-cache'),
    app = express();

var myLimit = typeof (process.argv[2]) != 'undefined' ? process.argv[2] : '100kb';
console.log('Using limit: ', myLimit);

app.use(bodyParser.json({ limit: myLimit }));

const agentOptions = {
    host: 'balticlsc.iem.pw.edu.pl',
    port: '443',
    path: '/',
    rejectUnauthorized: false
};

const agent = new https.Agent(agentOptions);

app.all('*', function (req, res, next) {

    // Set CORS headers: allow all origins, methods, and headers: you may want to lock this down in a production environment
    res.header("Access-Control-Allow-Origin", "*");
    res.header("Access-Control-Allow-Methods", "GET, PUT, PATCH, POST, DELETE");
    res.header("Access-Control-Allow-Headers", req.header('access-control-request-headers'));

    if (req.method === 'OPTIONS') {
        // CORS Preflight
        res.send();
    } else {
        const targetURL = req.header('Target-URL');
        if (!targetURL) {
            res.send(500, { error: 'There is no Target-Endpoint header in the request' });
            return;
        }

        const key = "__cache__" + targetURL + req.method

        // Cache for performance tests
        const cachedResponse = cache.get(key)

        if (cachedResponse) {
            console.log("Response generated from cache")
            res.send(cachedResponse.body)
            return
        }

        request({ url: targetURL, method: req.method, agent, json: req.body, headers: { 'Authorization': req.header('Authorization') || '' } },
            function (error, response, body) {
                if (error) {
                    console.error('error: ' + response?.statusCode)
                }

                if (res.statusCode !== 401) {
                    cache.put(key, response, 1000 * 60 * 60)
                }
            }).pipe(res);
    }
});

app.set('port', process.env.PORT || 3000);

app.listen(app.get('port'), function () {
    console.log('Proxy server listening on port ' + app.get('port'));
});