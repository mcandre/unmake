PKG=curl
CMD1=apt-get install -y $(PKG)
CMD2=apt-get install -y ${PKG}

CMD3::=apt-get install -y $(PKG)
CMD4:::=apt-get install -y $(PKG)
CMD5?=apt-get install -y $(PKG)

CMD6=
CMD6+=apt-get install -y $(PKG)
