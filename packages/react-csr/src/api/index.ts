import axios from "axios";

const HOST = "https://balticlsc.iem.pw.edu.pl/backend/app";
const UNAUTHORIZED_HOST = "https://balticlsc.iem.pw.edu.pl/backend";
const PROXY_URL = "http://localhost:3000/";
const AUTH_TOKEN = "AUTH_TOKEN";
const RETRY_STATUS = "RETRY_STATUS";
const USERNAME = "demo";
const PASSWORD = "BalticDemo";

export enum RequestMethod {
  GET = "get",
  POST = "post",
  PUT = "put",
  PATCH = "patch",
  DELETE = "delete",
}

const getAuthToken = () => localStorage.getItem(AUTH_TOKEN);
const setAuthToken = (token: string) => {
  localStorage.setItem(AUTH_TOKEN, token);
};

const getRetryStatus = () => parseInt(localStorage.getItem(RETRY_STATUS) || "0");
const setRetryStatus = (value: number) => {
  localStorage.setItem(RETRY_STATUS, value.toString());
};

const API = axios.create();

API.interceptors.request.use((config) => {
  config.headers["Authorization"] = `Bearer ${getAuthToken()}`;
  return config;
});

API.interceptors.response.use(
  (config) => {
    setRetryStatus(0);
    return config;
  },
  (error) => {
    const shouldRetry = getRetryStatus() < 3;

    if (error.response.status === 401 && shouldRetry) {
      sendRequest(RequestMethod.POST, "/Login", { password: PASSWORD, username: USERNAME }, true).then((res) => {
        setAuthToken(res.token);
        console.log("login", res);
        setRetryStatus(getRetryStatus() + 1);
      });
    }

    return Promise.reject(error);
  }
);

export const sendRequest = (method: RequestMethod, url: string, data?: any, unauthorized?: boolean) =>
  API({
    method,
    url: PROXY_URL,
    data,
    headers: {
      "Target-URL": `${unauthorized ? UNAUTHORIZED_HOST : HOST}${url}`,
      "Content-Type": "application/json",
    },
  }).then((response) => response.data.data || {});
