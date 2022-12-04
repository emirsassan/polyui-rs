import THREE from 'three';
import { cubes } from '../utils'

export async function getPolyModel() {
	const greyscale = new Uint8Array(4);
	greyscale[0] = 64;
	greyscale[1] = 64;
	greyscale[2] = 64;
	greyscale[3] = 255;
	const material = new THREE.MeshLambertMaterial({
		color: 0xffffff,
		map: await getTexture('assets/player_skin.png'),
		alphaMap: new THREE.DataTexture(greyscale, 1, 1),
		transparent: true
	});
	const model = new THREE.Object3D();
	model.name = 'polyui_player_model_guide';
	model.position.x = 0.001;
	model.position.y = 0.001;
	model.position.z = 0.001;
    
    for (const cube of cubes) {
        const mesh = new THREE.Mesh(new THREE.BoxGeometry(...cube.size), material)
        if (cube.origin) {
          mesh.position.set(cube.origin[0], cube.origin[1], cube.origin[2])
          mesh.geometry.translate(-cube.origin[0], -cube.origin[1], -cube.origin[2])
        }
        mesh.geometry.translate(cube.pos[0], cube.pos[1], cube.pos[2])
        for (const [key, face] of Object.entries(cube)) {
          if (face.uv !== undefined) {
            let fIndex = 0;
            switch(key) {
              case "north": fIndex = 10; break
              case "east": fIndex = 0; break
              case "south": fIndex = 8; break
              case "west": fIndex = 2; break
              case "up": fIndex = 4; break
              case "down": fIndex = 6; break
            }
            const uv_array = [
              [face.uv[0] / 16, 1 - (face.uv[1] / 16)],
              [face.uv[2] / 16, 1 - (face.uv[1] / 16)],
              [face.uv[0] / 16, 1 - (face.uv[3] / 16)],
              [face.uv[2] / 16, 1 - (face.uv[3] / 16)]
            ]
            mesh.geometry.attributes.uv = mesh.geometry.attributes.uv as THREE.BufferAttribute
            mesh.geometry.attributes.uv.set(uv_array[0], fIndex * 4 + 0)
            mesh.geometry.attributes.uv.set(uv_array[1], fIndex * 4 + 2)
            mesh.geometry.attributes.uv.set(uv_array[2], fIndex * 4 + 4)
            mesh.geometry.attributes.uv.set(uv_array[3], fIndex * 4 + 6)
            mesh.geometry.attributes.uv.needsUpdate = true
          }
        }
        model.add(mesh)
      }
      if (Settings.get("display_skin")) {
        let val = Settings.get("display_skin") as string
        if (val.startsWith("username:")) {
          fetch(`https://api.mojang.com/users/profiles/minecraft/${val.slice(9)}`).then(async r => {
            const uuid = await r.json()
            if (uuid?.id) {
              fetch(`https://sessionserver.mojang.com/session/minecraft/profile/${uuid.id}`).then(async r => {
                const data = await r.json()
                model.children[0].material.map = await getTexture(JSON.parse(Buffer.from(data.properties[0].value, "base64").toString()).textures.SKIN.url)
              })
            }
          })
        } else {
          if (val.slice(1, 2) === ",") val = val.slice(2)
          try {
            model.children[0].material.map = await getTexture(val)
          } catch(err) {
            console.log(err)
          }
        }
      }
}

export async function getTexture(path: string) {
	const texture = (await new Promise((fulfill) =>
		new THREE.TextureLoader().load(path, fulfill, undefined, fulfill)
	)) as unknown as THREE.Texture;
	const canvas = document.createElement('canvas');
	canvas.width = 64;
	canvas.height = 64;
	const ctx = canvas.getContext('2d');
	ctx?.drawImage(texture.image, 0, 0, 32, 16, 0, 0, 32, 16);
	ctx?.drawImage(texture.image, 8, 16, 4, 4, 8, 16, 4, 4);
	ctx?.drawImage(texture.image, 0, 20, 8, 12, 0, 20, 8, 12);
	ctx?.drawImage(texture.image, 12, 20, 4, 20, 12, 20, 4, 20);
	ctx?.drawImage(texture.image, 20, 16, 8, 4, 20, 16, 8, 4);
	ctx?.drawImage(texture.image, 20, 20, 8, 12, 20, 20, 8, 12);
	ctx?.drawImage(texture.image, 32, 20, 8, 12, 32, 20, 8, 12);
	ctx?.drawImage(texture.image, 44, 16, 8, 4, 44, 16, 8, 4);
	ctx?.drawImage(texture.image, 40, 20, 8, 12, 40, 20, 8, 12);
	ctx?.drawImage(texture.image, 52, 20, 4, 12, 52, 20, 4, 12);
	ctx?.drawImage(texture.image, 24, 48, 4, 4, 24, 48, 4, 4);
	ctx?.drawImage(texture.image, 20, 52, 12, 12, 20, 52, 12, 12);
	ctx?.drawImage(texture.image, 36, 48, 8, 4, 36, 48, 8, 4);
	ctx?.drawImage(texture.image, 36, 52, 12, 12, 36, 52, 12, 12);
	const skin = new THREE.CanvasTexture(canvas);
	return skin;
}
