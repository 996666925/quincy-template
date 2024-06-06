import { FilterPattern, PluginOption, createFilter, defineConfig } from 'vite'
import { readdirSync, lstatSync, copyFile } from 'fs';
import { join } from 'path';

function quincy(input: { include?: FilterPattern, exclude?: FilterPattern } = {}): PluginOption {
  const filter = createFilter(input.include, input.exclude);
  return {
    name: 'quincy',
    transform(code, id) {
      if (!filter(id))
        return null;

      if (id.includes("src/index.ts")) {
        let exports = traverseDir("./src").map((path) => `export * from '${path}'`)
        return exports.join("\n");
      }

      let script = id.split("/").pop();

      let result = code.match(/class\s+(.*?)\s+extends\s+Component\s+{/);
      if (!result) {
        return;
      }
      let clazz = result[1];
      code = code.replace(/\s+extends\s+Component\s+{/, ` extends Component {
          static {
            this.typeName="${clazz}";
            globalThis.__${clazz}__=new ${clazz}();
            globalThis['##${clazz}##']=()=>new ${clazz}();
            globalThis.__Components__.push({name:"${clazz}",script:"${script}"})
          }`)
      // console.log(code);
      return code;
    }, buildEnd() {
      copyFile("./project.json", "./assets/project.json", () => { })

    }


  }
}

// 递归遍历目录并获取所有文件路径  
function traverseDir(dirPath: string, fileList: string[] = []) {
  const files = readdirSync(dirPath);
  files.forEach(file => {
    const fullPath = join(dirPath, file);
    const stats = lstatSync(fullPath);
    if (stats.isDirectory()) {
      fileList = traverseDir(fullPath, fileList);
    } else if (fullPath.includes(".ts")) {
      fileList.push(fullPath);
    }
  });
  return fileList.map((path) => path.replace("src", ".").replace(".ts", "").replace(/\\/g, "/"));
}


export default defineConfig({
  build: {

    lib: {
      entry: ['./src/index.ts'],
      name: 'Quincy',
      fileName: 'quincy',

    },
    target: "esnext",
    outDir: "./assets/dist"
  },
  plugins: [quincy({ include: "./src/**/**.ts" })],

})

