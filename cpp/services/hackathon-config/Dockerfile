FROM ghcr.io/microsoft/vcpkg:latest AS build
WORKDIR /app

COPY cpp /app/cpp

RUN cmake -S cpp/services/hackathon-config -B /app/build -G Ninja \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_TOOLCHAIN_FILE=/usr/local/vcpkg/scripts/buildsystems/vcpkg.cmake
RUN cmake --build /app/build

FROM ghcr.io/microsoft/vcpkg:latest AS runtime
WORKDIR /app

COPY --from=build /app/build/hackathon_config /app/hackathon_config
EXPOSE 8080

CMD ["./hackathon_config"]
