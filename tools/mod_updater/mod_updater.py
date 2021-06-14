#! `env python3`
# Currently in python because it's a prototype
import os;

def walk(path, args):
    for (dirpath, dirnames, filenames) in os.walk(path):
        filenames.sort()
        dirnames.sort()
        # get the original mod file.
        mod_file = "{}/mod.rs".format(dirpath)
        if os.path.exists("{}/lib.rs".format(dirpath)):
            mod_file = "{}/lib.rs".format(dirpath)
        if not os.path.exists(mod_file):
                print("!!! No mod file found for {}",dirpath)
                # TODO: create the file from a template.
        print("Processing {}".format(mod_file))
        output = ""
        skip = []
        # Read the content of the mod file and filter out the
        # `pub mod ...;` lines. This will preserve things like crate
        # mods, comments, etc.
        try:
            with open(mod_file, "r") as file:
                blank = False
                for line in file.readlines():
                    # skip extra blank lines:
                    if line == "":
                        if blank:
                            continue
                        else:
                            blank = True
                    else:
                        blank = False
                    if not line.startswith("pub mod"):
                        # Special mods are generally marked as `pub(crate) mod...`
                        if line.startswith("pub(crate) mod"):
                            # we want to skip adding the special crates if they show
                            # up in the local list
                            cand = line.rsplit(" ",1)[1].replace(";\n", "").replace("r#", "")
                            # print("Skipping {}".format(cand))
                            skip.append(cand)
                        output += line
                        continue
        except FileNotFoundError as ex:
            print("!!! {} not found!".format(mod_file))
            pass
        # now append the new bits
        for name in filenames:
            # stupid sanity check.
            if name.endswith(".rs"):
                mod_name = name.split(".")[0]
                # Also really want to skip adding ourselves.
                if mod_name in ["mod", "lib"] or mod_name in skip:
                    continue
                # print ("adding mod {}".format(mod_name))
                output += "pub mod {};\n".format(mod_name)
        for name in dirnames:
            if name.startswith('.') or name in skip:
                continue
            # print ("adding mod {}".format(name))
            output += "pub mod {};\n".format(name)
        with open(mod_file, "w") as file:
            file.write(output)


def get_args():
    args = dict()
    for arg in os.environ.keys():
        PREFIX="MOD_UP_"
        if arg.startswith(PREFIX):
            print("Found.. {}".format(arg))
            args[arg[len(PREFIX):].upper()] = os.environ.get(arg)
    return args

def main():
    args = get_args()
    walk(args.get("PATH", "../../googleapis-raw/src/"), args)

main()
