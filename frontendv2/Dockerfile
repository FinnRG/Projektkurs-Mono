FROM bufbuild/buf:1.4.0 as buf
FROM node:lts-alpine as builder

WORKDIR /app

COPY ./package.json ./
RUN npm install

COPY . .
RUN npm run build

FROM nginx:stable-alpine

COPY --from=buf /usr/local/bin/buf /usr/local/bin/buf
COPY buf.work.yaml buf.work.yaml
COPY buf.gen.yaml buf.gen.yaml
RUN buf generate

COPY --from=builder /app/dist /usr/share/nginx/html
RUN rm /etc/nginx/conf.d/default.conf
COPY nginx/nginx.conf /etc/nginx/conf.d
EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]