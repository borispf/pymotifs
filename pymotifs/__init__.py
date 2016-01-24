def hello():
    import rustmotifs_binding
    return rustmotifs_binding.run([
        [0,1,2],
        [0,0,-1],
        [0,0,0]
    ])

