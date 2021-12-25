// As stated in https://stackoverflow.com/questions/64402821/module-not-found-error-cant-resolve-util-in-webpack
module.exports = {
    // ...
    resolve: {
        fallback: {
            util: require.resolve("util/")
        }
    }
    // ...
};