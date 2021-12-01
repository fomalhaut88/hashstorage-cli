(async () => {
    // Import
    const hsc = await import('hashstorage-cli')
    console.log(hsc)

    // Working with Api object
    const api = hsc.Api.new("http://localhost:8000")

    const version = await api.getVersion()
    console.log(version)

    // Working with profile
    const profile = hsc.Profile.new("appidstring", "alex", "Qwerty123")
    console.log(profile)
    const groups = await profile.getGroups(api)
    console.log(groups)

    // Save and load profile in LocalStorage
    profile.save()
    const profile2 = hsc.Profile.load()
    console.log(profile2, profile2.check())

    // Create and save block
    let block = hsc.Block.new(profile.publicKey(), "mygroup", "mykey")
    block.setData("Hello world")
    await block.save(api, profile)

    // Get and update block
    let blockJson = await profile.getBlockJson(api, "mygroup", "mykey")
    let block2 = hsc.Block.fromBlockJson(blockJson)
    block2.setData("Hi")
    await block2.save(api, profile)
    console.log(block2, block2.version())
})();
