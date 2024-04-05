import os
import sys
import requests

def main(project_token: str):
    url = sys.argv[1]

    headers = {
        "PRIVATE-TOKEN": project_token
    }

    response = requests.get(url, headers=headers)
    with open("ISSUES.md", "w") as f:
        for issue in response.json():
            id = issue["id"]
            internal_id = issue["iid"]
            title = issue["title"]
            desc = issue["description"]
            notes = []
            u = f"{url}/{internal_id}/notes"
            res = requests.get(u, headers=headers)
        
            for comments in res.json():
                notes.append(comments["body"])

            f.write(f"id: {id}\n")
            f.write(f"iid: {internal_id}\n\n")
            f.write(f"# {title}\n\n")
            f.write(f"{desc}\n")
            f.writelines(notes)
            f.write(f"\nurl:{u}\n\n")
            f.write("-" * 60 + "\n")

if __name__ == "__main__":
    project_token = os.environ.get('PROJECT_TOKEN')

    if project_token is None:
        print("Project token not found.")
        exit(1)

    main(project_token)

