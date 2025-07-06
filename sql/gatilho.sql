CREATE OR REPLACE FUNCTION is_max_workspaces_created_reached(user_id uuid)
RETURNS BOOLEAN
LANGUAGE SQL
AS $$
    SELECT u.numCreatedWorkspaces >= st.MaxWorkspaces
    FROM Users u
    JOIN SubscriptionTypes st ON st.SubscriptionTypeId = u.SubscriptionTypeId
    WHERE u.UserId = user_id;
$$;

CREATE OR REPLACE PROCEDURE update_user_workspace_count(user_id UUID)
LANGUAGE SQL
AS $$
    UPDATE Users
    SET numCreatedWorkspaces = numCreatedWorkspaces + 1
    WHERE UserId = user_id;
$$;


CREATE OR REPLACE FUNCTION update_user_num_workspaces()
  RETURNS TRIGGER
  LANGUAGE PLPGSQL
  AS
$$
BEGIN
	IF is_max_workspaces_created_reached(NEW.OwnerUserId) THEN
        RAISE EXCEPTION 'User % has reached the maximum number of workspaces allowed.', NEW.OwnerUserId;
	END IF;

    CALL update_user_workspace_count(NEW.OwnerUserId);

	RETURN NEW;
END;
$$;

CREATE OR REPLACE TRIGGER trg_update_user_num_workspaces
    BEFORE INSERT ON Workspaces
    FOR EACH ROW
    EXECUTE FUNCTION update_user_num_workspaces();
