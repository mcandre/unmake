PKG=curl
CMD1=apt install -y $(PKG)
CMD2=apt install -y ${PKG}

CMD3::=apt install -y $(PKG)
CMD4:::=apt install -y $(PKG)
CMD5?=apt install -y $(PKG)

CMD6=
CMD6+=apt install -y $(PKG)
