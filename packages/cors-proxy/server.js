const express = require('express'),
    rp = require('request-promise'),
    bodyParser = require('body-parser'),
    https = require('https'),
    app = express(),
    redis = require('redis');

const USERNAME = "demo";
const PASSWORD = "BalticDemo";
const myLimit = typeof (process.argv[2]) != 'undefined' ? process.argv[2] : '100kb';
const authorizationCacheKey = '__cache__Authorization';
let redisClient;

(async () => {
    redisClient = redis.createClient();
    redisClient.on("error", (error) => console.error(`Error : ${error}`));
    await redisClient.connect();
})();

app.use(bodyParser.json({ limit: myLimit }));

const agentOptions = {
    host: 'balticlsc.iem.pw.edu.pl',
    port: '443',
    path: '/',
    rejectUnauthorized: false
};

const agent = new https.Agent(agentOptions);


const getAuthKey = () => new Promise((resolve) => {
    rp({
        agent,
        method: 'POST',
        url: "https://balticlsc.iem.pw.edu.pl/backend/Login",
        body: { password: PASSWORD, username: USERNAME },
        json: true
    })
        .then(res => resolve({ token: `Bearer ${res.data?.token}`, err: null }))
        .catch(err => resolve({ token: null, err }))
})

const getProxyRequest = (method, url, json, token) => new Promise(async (resolve) => {
    const headers = { Authorization: token }

    rp({ agent, method, url, json, headers })
        .then(res => resolve({ res, err: null }))
        .catch(err => resolve({ res: null, err }))
})


app.all('*', async function (req, res, next) {


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
        const cacheResults = await redisClient.get(key);

        if (cacheResults) {
            console.log("Response generated from cache")
            const results = JSON.parse(cacheResults);
            res.send(results)
        } else {
            const authKey = await redisClient.get(authorizationCacheKey);

            if (!authKey) {
                const { token, err } = await getAuthKey()

                if (token) {
                    await redisClient.set(authorizationCacheKey, token);
                    const { res: proxyRes, err: proxyErr } = await getProxyRequest(req.method, targetURL, req.body, token)

                    if (proxyErr) {
                        res.send(500, { error: err })
                    } else {
                        await redisClient.set(key, JSON.stringify(proxyRes));
                        res.send(proxyRes)
                    }
                } else {
                    res.send(401, { error: err })
                }
            } else {
                const cachedToken = await redisClient.get(authorizationCacheKey);
                const { res: proxyRes, err: proxyErr } = await getProxyRequest(req.method, targetURL, req.body, cachedToken)

                if (proxyErr) {
                    const { token, err: authErr } = await getAuthKey()

                    if (token) {
                        await redisClient.set(authorizationCacheKey, token);
                        const { res: proxyRes, err: proxyErr } = await getProxyRequest(req.method, targetURL, req.body, token)

                        if (proxyErr) {
                            res.send(500, { error: proxyErr })
                        } else {
                            res.send(proxyRes)
                        }
                    } else {
                        res.send(401, { error: proxyErr || authErr })
                    }
                } else {
                    await redisClient.set(key, JSON.stringify(proxyRes));
                    res.send(proxyRes)
                }
            }
        }
    }
});

app.set('port', process.env.PORT || 3000);

app.listen(app.get('port'), function () {
    console.log('Proxy server listening on port ' + app.get('port'));
});