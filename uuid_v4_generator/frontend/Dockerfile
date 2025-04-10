FROM node:23.11.0-slim AS builder 

WORKDIR /app 

COPY package.json ./
RUN npm install -g bun && bun install 

COPY . .
RUN bun run build 

FROM nginx:stable-alpine3.20-slim

RUN apk add --no-cache curl && rm -f /etc/nginx/conf.d/default.conf

WORKDIR /app

COPY nginx.conf /etc/nginx/nginx.conf

COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]