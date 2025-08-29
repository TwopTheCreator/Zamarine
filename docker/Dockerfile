FROM debian:bookworm

ENV DEBIAN_FRONTEND=noninteractive \
    LC_ALL=C.UTF-8 \
    LANG=C.UTF-8

RUN apt-get update && apt-get install -y --no-install-recommends \
    live-build \
    debootstrap \
    apt-transport-https \
    ca-certificates \
    git curl wget xz-utils \
    squashfs-tools \
    syslinux isolinux \
    dosfstools \
    cpio \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace

COPY live-build /workspace/live-build

RUN chmod +x /workspace/live-build/auto/config \
    && chmod +x /workspace/live-build/auto/build \
    && mkdir -p /workspace/out

CMD ["/workspace/live-build/auto/build"]


