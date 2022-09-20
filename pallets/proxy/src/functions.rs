use super::*;
use frame_support::{pallet_prelude::*};
use frame_support::traits::Time;
use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec; // vec primitive
use scale_info::prelude::vec; // vec![] macro

use pallet_rbac::types::*;
use crate::types::*;

impl<T: Config> Pallet<T> {
    // M A I N  F U N C T I O N S
    // --------------------------------------------------------------------------------------------
    
    pub fn do_initial_setup() -> DispatchResult{
        let pallet_id = Self::pallet_id();
        let global_scope = pallet_id.using_encoded(blake2_256);
        <GlobalScope<T>>::put(global_scope);

        //Admin rol & permissions
        let administrator_role_id = T::Rbac::create_and_set_roles(pallet_id.clone(), [ProxyRole::Administrator.to_vec()].to_vec())?;
        T::Rbac::create_and_set_permissions(pallet_id.clone(), administrator_role_id[0], ProxyPermission::administrator_permissions())?;
        
        //Developer rol & permissions
        let _developer_role_id = T::Rbac::create_and_set_roles(pallet_id.clone(), [ProxyRole::Developer.to_vec()].to_vec())?;
        //T::Rbac::create_and_set_permissions(pallet_id.clone(), developer_role_id[0], ProxyPermission::developer_permissions())?;

        // Investor rol & permissions
        let _investor_role_id = T::Rbac::create_and_set_roles(pallet_id.clone(), [ProxyRole::Investor.to_vec()].to_vec())?;
        //T::Rbac::create_and_set_permissions(pallet_id.clone(), investor_role_id[0], ProxyPermission::investor_permissions())?;

        // Issuer rol & permissions
        let _issuer_role_id = T::Rbac::create_and_set_roles(pallet_id.clone(), [ProxyRole::Issuer.to_vec()].to_vec())?;
        //T::Rbac::create_and_set_permissions(pallet_id.clone(), issuer_role_id[0], ProxyPermission::issuer_permissions())?;

        // Regional center rol & permissions
        let _regional_center_role_id = T::Rbac::create_and_set_roles(pallet_id.clone(), [ProxyRole::RegionalCenter.to_vec()].to_vec())?;
        //T::Rbac::create_and_set_permissions(pallet_id.clone(), regional_center_role_id[0], ProxyPermission::regional_center_permissions())?;
        
        // Create a global scope for the administrator role
        T::Rbac::create_scope(Self::pallet_id(), global_scope)?;

        Self::deposit_event(Event::ProxySetupCompleted);
        Ok(())
    }

    pub fn do_sudo_add_administrator(
        admin: T::AccountId, 
    ) -> DispatchResult{
        let pallet_id = Self::pallet_id();
        let global_scope =  <GlobalScope<T>>::try_get().map_err(|_| Error::<T>::GlobalScopeNotSet)?;

        T::Rbac::assign_role_to_user(
            admin.clone(), 
            pallet_id.clone(), 
            &global_scope, 
            ProxyRole::Administrator.id())?;

        //TODO: create a administrator user account
        //Self::do_register_user(admin, user, documents)
        
        Self::deposit_event(Event::AdministratorAssigned(admin));
        Ok(())
    }

    pub fn do_sudo_remove_administrator(
        admin: T::AccountId, 
    ) -> DispatchResult{
        let pallet_id = Self::pallet_id();
        let global_scope =  <GlobalScope<T>>::try_get().map_err(|_| Error::<T>::GlobalScopeNotSet)?;

        T::Rbac::remove_role_from_user(
            admin.clone(), 
            pallet_id.clone(), 
            &global_scope, 
            ProxyRole::Administrator.id())?;
        
        //TODO: remove administrator user account
        //Self::do_delete_user(admin, user, documents)
        
        Self::deposit_event(Event::AdministratorRemoved(admin));
        Ok(())
    }
    
    pub fn do_create_project(
        admin: T::AccountId, 
        tittle: BoundedVec<u8, T::ProjectNameMaxLen>,
        description: BoundedVec<u8, T::ProjectDescMaxLen>,
        image: CID,
        completition_date: u64,
        ) -> DispatchResult {
        //ensure admin permissions 
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //Add timestamp 
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        //Create project_id
        //TOREVIEW: We could use only name as project_id or use a method/storagemap to check if the name is already in use
        let project_id = (tittle.clone()).using_encoded(blake2_256);

        //ensure completition date is in the future
        ensure!(completition_date > timestamp, Error::<T>::CompletitionDateMustBeLater);
        
        //Create project data
        let project_data = ProjectData::<T> {
            developer: Some(BoundedVec::<T::AccountId, T::MaxDevelopersPerProject>::default()),
            investor: Some(BoundedVec::<T::AccountId, T::MaxInvestorsPerProject>::default()),
            issuer: Some(BoundedVec::<T::AccountId, T::MaxIssuersPerProject>::default()),
            regional_center: Some(BoundedVec::<T::AccountId, T::MaxRegionalCenterPerProject>::default()),
            tittle,
            description,
            image,
            status: ProjectStatus::default(), 
            creation_date: timestamp,
            completition_date,
            updated_date: timestamp,
        };

        //Insert project data
        // ensure that the project_id is not already in use
        ensure!(!ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectIdAlreadyInUse);
        ProjectsInfo::<T>::insert(project_id, project_data);

        //TODO: create scope for project_id
        T::Rbac::create_scope(Self::pallet_id(), project_id)?;

        // Event
        Self::deposit_event(Event::ProjectCreated(admin, project_id));

        Ok(())
    }

    pub fn do_edit_project(
        admin: T::AccountId,
        project_id: [u8;32], 
        tittle: Option<BoundedVec<u8, T::ProjectNameMaxLen>>,
        description: Option<BoundedVec<u8, T::ProjectDescMaxLen>>, 
        image: Option<CID>, 
        creation_date: Option<u64>, 
        completition_date: Option<u64>,  
    ) -> DispatchResult {
        //ensure admin permissions             
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;
        
        //Ensure project exists & get project data
        let project_data = ProjectsInfo::<T>::get(project_id).ok_or(Error::<T>::ProjectNotFound)?;

        // Ensure project is not completed
        ensure!(project_data.status != ProjectStatus::Completed, Error::<T>::CannotEditCompletedProject);

        //Get current timestamp
        let current_timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        //Mutate project data
        <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
            let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
            
            if let Some(tittle) = tittle {
                project.tittle = tittle;
            }
            if let Some(description) = description {
                project.description = description;
            }
            if let Some(image) = image {
                project.image = image;
            }
            if let Some(creation_date) = creation_date {
                //ensure new creation date is in the past
                ensure!(creation_date < current_timestamp, Error::<T>::CreationDateMustBeInThePast);
                project.creation_date = creation_date;
            }
            if let Some(completition_date) = completition_date {
                //ensure new completition date is in the future
                ensure!(completition_date > current_timestamp, Error::<T>::CompletitionDateMustBeLater);
                project.completition_date = completition_date;
            }

            Ok(())    
        })?;

        // Event
        Self::deposit_event(Event::ProjectEdited(project_id));
        Ok(())
    } 

    pub fn do_delete_project(
        admin: T::AccountId,
        project_id: [u8;32], 
    ) -> DispatchResult {
        //ensure admin permissions 
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //Ensure project exists & get project data
        let project_data = ProjectsInfo::<T>::get(project_id).ok_or(Error::<T>::ProjectNotFound)?;

        //Ensure project is not completed
        ensure!(project_data.status != ProjectStatus::Completed, Error::<T>::CannotDeleteCompletedProject);

        // Delete scope from rbac pallet
        T::Rbac::remove_scope(Self::pallet_id(), project_id)?;

        //TOREVIEW: check if this method is the best way to delete data from storage
        // we could use get method (<UsersByProject<T>>::get()) instead getter function
        // Delete project from ProjectsByUser storage
        let users_by_project = Self::users_by_project(project_id).iter().cloned().collect::<Vec<T::AccountId>>();
        for user in users_by_project {
            <ProjectsByUser<T>>::mutate(user, |projects| {
                projects.retain(|project| *project != project_id);
            });
        }

        // Delete from ProjectsInfo storagemap
        <ProjectsInfo<T>>::remove(project_id);

        // Delete from UsersByProject storagemap
        <UsersByProject<T>>::remove(project_id);

        //Event 
        Self::deposit_event(Event::ProjectDeleted(project_id));
        Ok(())
    }


    pub fn do_register_user(admin: T::AccountId, user: T::AccountId, name: FieldName, image: CID, email: FieldName, documents: Option<Documents<T>>, ) -> DispatchResult {
        //ensure admin permissions     
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        // check if user is already registered
        ensure!(!<UsersInfo<T>>::contains_key(user.clone()), Error::<T>::UserAlreadyRegistered);

        //Get current timestamp
        let current_timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        let user_data = UserData::<T> {
            name,
            role: None,
            image,
            date_registered: current_timestamp,
            email,
            documents,
        };

        //Insert user data
        <UsersInfo<T>>::insert(user.clone(), user_data);
        Self::deposit_event(Event::UserAdded(user));
        Ok(())
    }

    pub fn do_assign_user(
        admin: T::AccountId,
        user: T::AccountId,
        project_id: [u8;32], 
        role: ProxyRole, 
    ) -> DispatchResult {
        //ensure admin permissions 
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //Ensure project exists & get project data
        let project_data = ProjectsInfo::<T>::get(project_id).ok_or(Error::<T>::ProjectNotFound)?;

        //Ensure project is not completed
        ensure!(project_data.status != ProjectStatus::Completed, Error::<T>::CannotEditCompletedProject);

        //Ensure user is registered
        ensure!(<UsersInfo<T>>::contains_key(user.clone()), Error::<T>::UserNotRegistered);

        //Ensure user is not already assigned to the project
        ensure!(!<UsersByProject<T>>::get(project_id).contains(&user.clone()), Error::<T>::UserAlreadyAssignedToProject);
        ensure!(!<ProjectsByUser<T>>::get(user.clone()).contains(&project_id), Error::<T>::UserAlreadyAssignedToProject);

        //TODO: Ensure user is not assigened to the selected scope (project_id) with the selected role
        ensure!(!T::Rbac::has_role(user.clone(), Self::pallet_id(), &project_id, [role.id()].to_vec()).is_ok(), Error::<T>::UserAlreadyHasRole);


        //TODO:Update project data depending on the role assigned
        Self::update_project_role(project_id, user.clone(), role)?;

        //TOREVIEW: this storage map will be removed?
        // Insert project to ProjectsByUser storagemap
        <ProjectsByUser<T>>::try_mutate::<_,_,DispatchError,_>(user.clone(), |projects| {
            projects.try_push(project_id).map_err(|_| Error::<T>::MaxProjectsPerUserReached)?;
            Ok(())
        })?;

        //TOREVIEW: this storage map will be removed?
        // Insert user to UsersByProject storagemap
        <UsersByProject<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |users| {
            users.try_push(user.clone()).map_err(|_| Error::<T>::MaxUsersPerProjectReached)?;
            Ok(())
        })?;

        // Insert user into scope rbac pallet
        T::Rbac::assign_role_to_user(user.clone(), Self::pallet_id(), &project_id, role.id())?;

        //Event 
        Self::deposit_event(Event::UserAssignedToProject(user, project_id));
        Ok(())
    }

    pub fn do_unassign_user(
        admin: T::AccountId,
        user: T::AccountId,
        project_id: [u8;32], 
        role: ProxyRole, 
    ) -> DispatchResult {
        //ensure admin permissions 
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //Ensure project exists & get project data
        let project_data = ProjectsInfo::<T>::get(project_id).ok_or(Error::<T>::ProjectNotFound)?;

        //Ensure project is not completed
        ensure!(project_data.status != ProjectStatus::Completed, Error::<T>::CannotEditCompletedProject);

        //Ensure user is registered
        ensure!(<UsersInfo<T>>::contains_key(user.clone()), Error::<T>::UserNotRegistered);

        //Ensure user is assigned to the project
        ensure!(<UsersByProject<T>>::get(project_id).contains(&user.clone()), Error::<T>::UserNotAssignedToProject);
        ensure!(<ProjectsByUser<T>>::get(user.clone()).contains(&project_id), Error::<T>::UserNotAssignedToProject);

        // Ensure user has roles assigned to the project
        T::Rbac::has_role(user.clone(), Self::pallet_id(), &project_id, [role.id()].to_vec())?;

        // Remove user from UsersByProject storagemap
        <UsersByProject<T>>::mutate(project_id, |users| {
            users.retain(|u| u != &user);
        });

        // Remove user from ProjectsByUser storagemap
        <ProjectsByUser<T>>::mutate(user.clone(), |projects| {
            projects.retain(|p| p != &project_id);
        });

        // Remove user from scope
        T::Rbac::remove_role_from_user(user.clone(), Self::pallet_id(), &project_id, role.id())?;

        Self::deposit_event(Event::UserUnassignedFromProject(user, project_id));
        Ok(())
    }

    pub fn do_update_user(
        admin: T::AccountId,
        user: T::AccountId,
        name: Option<FieldName>,
        image: Option<CID>,
        email: Option<FieldName>,
        documents: Option<Documents<T>>, 
    ) -> DispatchResult {
        //ensure admin permissions 
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //Ensure user is registered
        ensure!(<UsersInfo<T>>::contains_key(user.clone()), Error::<T>::UserNotRegistered);

        //Update user data
        <UsersInfo<T>>::try_mutate::<_,_,DispatchError,_>(user.clone(), |user_data| {
            let user_info = user_data.as_mut().ok_or(Error::<T>::UserNotRegistered)?;

            if let Some(name) = name {
                user_info.name = name;
            }
            if let Some(image) = image {
                user_info.image = image;
            }
            if let Some(email) = email {
                user_info.email = email;
            }
            if let Some(documents) = documents {
                user_info.documents = Some(documents);
            }
            Ok(())
        })?;

        Self::deposit_event(Event::UserUpdated(user));

        Ok(())
    }

    pub fn do_delete_user(
        admin: T::AccountId,
        user: T::AccountId,
    ) -> DispatchResult {
        //ensure admin permissions 
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //Ensure user is registered
        ensure!(<UsersInfo<T>>::contains_key(user.clone()), Error::<T>::UserNotRegistered);

        //Remove user from UsersInfo storage map
        <UsersInfo<T>>::remove(user.clone());

        //Remove user data from UsersByProject storage map
        let projects_by_user = Self::projects_by_user(user.clone()).iter().cloned().collect::<Vec<[u8;32]>>();
        for project_id in projects_by_user.clone() {
            <UsersByProject<T>>::mutate(project_id, |users| {
                users.retain(|u| u != &user);
            });
        }

        //Remove user from ProjectsByUser storage map
        <ProjectsByUser<T>>::remove(user.clone());


        //TODO: Optimise this
        //Remove user from its scopes
        // for scope in projects_by_user {
        //     //TODO: get each role from each scope instead guessing it
        //     //create a method in rbac pallet to get all roles from a scope
        //     T::Rbac::remove_role_from_user(user.clone(), Self::pallet_id(), &scope, ProxyRole::Developer.id())?;
        //     T::Rbac::remove_role_from_user(user.clone(), Self::pallet_id(), &scope, ProxyRole::Investor.id())?;
        //     T::Rbac::remove_role_from_user(user.clone(), Self::pallet_id(), &scope, ProxyRole::Issuer.id())?;
        //     T::Rbac::remove_role_from_user(user.clone(), Self::pallet_id(), &scope, ProxyRole::RegionalCenter.id())?;
        // }

        Self::deposit_event(Event::UserDeleted(user));

        Ok(())
    }

    // H E L P E R S
    // --------------------------------------------------------------------------------------------
    
    /// Get the current timestamp in milliseconds
    fn get_timestamp_in_milliseconds() -> Option<u64> {
        let timestamp:u64 = T::Timestamp::now().into();

        Some(timestamp)
    }

    /// Get the pallet_id
    pub fn pallet_id()->IdOrVec{
        IdOrVec::Vec(
            Self::module_name().as_bytes().to_vec()
        )
    }

    /// Get global scope
    pub fn get_global_scope() -> [u8;32] {
        let global_scope = <GlobalScope<T>>::try_get().map_err(|_| Error::<T>::NoneValue).unwrap();
        global_scope
    }


    fn change_project_status(project_id: [u8;32], status: ProjectStatus) -> DispatchResult {
        //ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        //Mutate project data
        <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
            let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
            project.status = status;
            Ok(())    
        })?;

        Ok(())
    }

    fn update_project_role(
        project_id: [u8;32],
        user: T::AccountId,
        role: ProxyRole,
    ) -> DispatchResult {

        match role {
            ProxyRole::Administrator => {
                return Err(Error::<T>::CannotRegisterAdminRole.into());
            },
            ProxyRole::Developer => {
                //Mutate project data
                <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
                    let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
                    match project.developer.clone(){
                        Some(mut developer) => {
                            //developer.iter().find(|&u| u == &user).ok_or(Error::<T>::UserAlreadyAssignedToProject)?;
                            developer.try_push(user.clone()).map_err(|_| Error::<T>::MaxDevelopersPerProjectReached)?;
                        },
                        None => {
                            let devs = project.developer.get_or_insert(BoundedVec::<T::AccountId, T::MaxDevelopersPerProject>::default());
                            devs.try_push(user.clone()).map_err(|_| Error::<T>::MaxDevelopersPerProjectReached)?;
                        }
                    }
                    Ok(())    
                })?;
            },
            ProxyRole::Investor => {
                //Mutate project data
                <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
                    let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
                    match project.investor.clone(){
                        Some(mut investor) => {
                            investor.iter().find(|&u| u == &user).ok_or(Error::<T>::UserAlreadyAssignedToProject)?;
                            investor.try_push(user.clone()).map_err(|_| Error::<T>::MaxInvestorsPerProjectReached)?;
                        },
                        None => {
                            let investors = project.investor.get_or_insert(BoundedVec::<T::AccountId, T::MaxInvestorsPerProject>::default());
                            investors.try_push(user.clone()).map_err(|_| Error::<T>::MaxInvestorsPerProjectReached)?;
                        }
                    }
                    Ok(())    
                })?;
            },
            ProxyRole::Issuer => {
                //Mutate project data
                <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
                    let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
                    match project.issuer.clone(){
                        Some(mut issuer) => {
                            issuer.iter().find(|&u| u == &user).ok_or(Error::<T>::UserAlreadyAssignedToProject)?;
                            issuer.try_push(user.clone()).map_err(|_| Error::<T>::MaxIssuersPerProjectReached)?;
                        },
                        None => {
                            let issuers = project.issuer.get_or_insert(BoundedVec::<T::AccountId, T::MaxIssuersPerProject>::default());
                            issuers.try_push(user.clone()).map_err(|_| Error::<T>::MaxIssuersPerProjectReached)?;
                        }
                    }
                    Ok(())    
                })?;
            },
            ProxyRole::RegionalCenter => {
                //Mutate project data
                <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
                    let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
                    match project.regional_center.clone(){
                        Some(mut regional_center) => {
                            regional_center.iter().find(|&u| u != &user).ok_or(Error::<T>::UserAlreadyAssignedToProject)?;
                            regional_center.try_push(user.clone()).map_err(|_| Error::<T>::MaxRegionalCenterPerProjectReached)?;
                        },
                        None => {
                            let regional_centers = project.regional_center.get_or_insert(BoundedVec::<T::AccountId, T::MaxRegionalCenterPerProject>::default());
                            regional_centers.try_push(user.clone()).map_err(|_| Error::<T>::MaxRegionalCenterPerProjectReached)?;
                        }
                    }
                    Ok(())    
                })?;
            },
        }

        Ok(())
    }

    fn is_authorized( authority: T::AccountId, project_id: &[u8;32], permission: ProxyPermission ) -> DispatchResult{
        T::Rbac::is_authorized(
            authority,
            Self::pallet_id(), 
            project_id,
            &permission.id(),
        )
    }

    fn is_superuser( authority: T::AccountId, scope_global: &[u8;32], rol_id: RoleId ) -> DispatchResult{
        T::Rbac::has_role(
            authority,
            Self::pallet_id(), 
            scope_global,
            vec![rol_id],
        )
    }




}